use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FromStr for Platform {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Platform, ParseError> {
        if let Some(s) = s.strip_prefix("cfg(").and_then(|s| s.strip_suffix(')')) {
            s.parse().map(Platform::Cfg)
        } else {
            Platform::validate_named_platform(s)?;
            Ok(Platform::Name(s.to_string()))
        }
    }
}
