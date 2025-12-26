use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Iterator over the entries in a directory.
///
/// This iterator is returned from [`Utf8Path::read_dir_utf8`] and will yield instances of
/// <code>[io::Result]<[Utf8DirEntry]></code>. Through a [`Utf8DirEntry`] information like the entry's path
/// and possibly other metadata can be learned.
///
/// The order in which this iterator returns entries is platform and filesystem
/// dependent.
///
/// # Errors
///
/// This [`io::Result`] will be an [`Err`] if there's some sort of intermittent
/// IO error during iteration.
///
/// If a directory entry is not UTF-8, an [`io::Error`] is returned with the
/// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`][io::ErrorKind::InvalidData]
/// and the payload set to a [`FromPathBufError`].
#[derive(Debug)]
pub struct ReadDirUtf8 {
    inner: fs::ReadDir,
}
