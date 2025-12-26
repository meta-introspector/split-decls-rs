use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Get the current IANA time zone as a string.
///
/// See the module-level documentation for a usage example and more details
/// about this function.
#[inline]
pub fn get_timezone() -> Result<String, GetTimezoneError> {
    platform::get_timezone_inner()
}
