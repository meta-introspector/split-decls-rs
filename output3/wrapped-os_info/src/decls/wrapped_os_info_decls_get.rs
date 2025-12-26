use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns information about the current operating system (type, version, edition, etc.).
///
/// # Examples
///
/// ```
/// use os_info;
///
/// let info = os_info::get();
///
/// // Print full information:
/// println!("OS information: {info}");
///
/// // Print information separately:
/// println!("Type: {}", info.os_type());
/// println!("Version: {}", info.version());
/// println!("Edition: {:?}", info.edition());
/// println!("Codename: {:?}", info.codename());
/// println!("Bitness: {}", info.bitness());
/// println!("Architecture: {:?}", info.architecture());
/// ```
pub fn get() -> Info {
    imp::current_platform()
}
