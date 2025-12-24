use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Entries returned by the [`ReadDirUtf8`] iterator.
///
/// An instance of [`Utf8DirEntry`] represents an entry inside of a directory on the filesystem. Each
/// entry can be inspected via methods to learn about the full path or possibly other metadata.
#[derive(Debug)]
pub struct Utf8DirEntry {
    inner: fs::DirEntry,
    path: Utf8PathBuf,
}
