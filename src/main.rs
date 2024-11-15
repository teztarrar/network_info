use reqwest::Error;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct GeoInfo {
    ip: Option<String>,     // Use Option to allow missing fields
    isp: Option<String>,
    country: Option<String>,
    region: Option<String>,
    city: Option<String>,
}

// Fetch geographic information
async fn get_geo_info() -> Result<GeoInfo, Error> {
    let response = reqwest::get("http://ip-api.com/json/")
        .await?
        .json::<GeoInfo>()
        .await?;

    Ok(response)
}

// Get the local IP address
fn get_local_ip() -> String {
    let output = Command::new("hostname")
        .arg("-I")
        .output()
        .expect("Failed to get local IP address");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

// Perform a speed test using the speedtest-cli command
fn run_speed_test() {
    println!("Running speed test...");
    let output = Command::new("speedtest")
        .arg("--simple")
        .output()
        .expect("Failed to run speed test");
    let result = String::from_utf8_lossy(&output.stdout);
    println!("{}", result);
}

#[tokio::main]
async fn main() {
    println!("Fetching internet information...\n");

    // Fetch geo info
    match get_geo_info().await {
        Ok(info) => {
            if let Some(ip) = info.ip {
                println!("Public IP Address: {}", ip);
            } else {
                println!("Public IP Address: Not available");
            }

            if let Some(isp) = info.isp {
                println!("ISP: {}", isp);
            } else {
                println!("ISP: Not available");
            }

            if let Some(location) = info.city {
                println!("Location: {}, {}", location, info.region.unwrap_or_default());
            } else {
                println!("Location: Not available");
            }
            
            if let Some(country) = info.country {
                println!("Country: {}", country);
            } else {
                println!("Country: Not available");
            }
        }
        Err(e) => eprintln!("Failed to fetch geo information: {:?}", e),
    }

    // Get local IP
    let local_ip = get_local_ip();
    println!("Local IP Address: {}\n", local_ip);

    // Run speed test
    run_speed_test();
}
