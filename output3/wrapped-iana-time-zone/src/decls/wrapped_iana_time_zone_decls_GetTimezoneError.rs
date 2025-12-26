use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error types
#[derive(Debug)]
pub enum GetTimezoneError {
    /// Failed to parse
    FailedParsingString,
    /// Wrapped IO error
    IoError(std::io::Error),
    /// Platform-specific error from the operating system
    OsError,
}
