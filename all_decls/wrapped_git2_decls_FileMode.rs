use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Valid modes for index and tree entries.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileMode {
    /// Unreadable
    Unreadable,
    /// Tree
    Tree,
    /// Blob
    Blob,
    /// Group writable blob. Obsolete mode kept for compatibility reasons
    BlobGroupWritable,
    /// Blob executable
    BlobExecutable,
    /// Link
    Link,
    /// Commit
    Commit,
}
