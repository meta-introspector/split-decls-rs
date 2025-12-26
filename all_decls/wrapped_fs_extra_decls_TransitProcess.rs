use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A structure which includes information about the current status of copying or moving a directory.
pub struct TransitProcess {
    /// Already copied bytes
    pub copied_bytes: u64,
    /// All the bytes which should be copied or moved (dir size).
    pub total_bytes: u64,
    /// Copied bytes on this time for file.
    pub file_bytes_copied: u64,
    /// Size of currently copied file.
    pub file_total_bytes: u64,
    /// Name of currently copied file.
    pub file_name: String,
    /// Name of currently copied folder.
    pub dir_name: String,
    /// Transit state
    pub state: dir::TransitState,
}
