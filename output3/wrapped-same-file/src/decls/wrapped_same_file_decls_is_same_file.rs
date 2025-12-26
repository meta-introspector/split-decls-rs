use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if the two file paths may correspond to the same file.
///
/// Note that it's possible for this to produce a false positive on some
/// platforms. Namely, this can return true even if the two file paths *don't*
/// resolve to the same file.
/// # Errors
/// This function will return an [`io::Error`] if any of the two paths cannot
/// be opened. The most common reasons for this are: the path does not exist,
/// or there were not enough permissions.
///
/// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
///
/// # Example
///
/// ```rust,no_run
/// use same_file::is_same_file;
///
/// assert!(is_same_file("./foo", "././foo").unwrap_or(false));
/// ```
pub fn is_same_file<P, Q>(path1: P, path2: Q) -> io::Result<bool>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    Ok(Handle::from_path(path1)? == Handle::from_path(path2)?)
}
