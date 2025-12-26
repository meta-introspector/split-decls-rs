use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The path of an existing writable directory in a system temporary directory.
///
/// Drop the struct to delete the directory and everything under it.
/// Deletes symbolic links and does not follow them.
///
/// Ignores any error while deleting.
/// See [`TempDir::panic_on_cleanup_error`](struct.TempDir.html#method.panic_on_cleanup_error).
///
/// # Example
/// ```rust
/// use temp_dir::TempDir;
/// let d = TempDir::new().unwrap();
/// // Prints "/tmp/t1a9b-0".
/// println!("{:?}", d.path());
/// let f = d.child("file1");
/// // Prints "/tmp/t1a9b-0/file1".
/// println!("{:?}", f);
/// std::fs::write(&f, b"abc").unwrap();
/// assert_eq!(
///     "abc",
///     std::fs::read_to_string(&f).unwrap(),
/// );
/// // Prints "/tmp/t1a9b-1".
/// println!("{:?}", TempDir::new().unwrap().path());
/// ```
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct TempDir {
    delete_on_drop: bool,
    panic_on_delete_err: bool,
    path_buf: PathBuf,
}
