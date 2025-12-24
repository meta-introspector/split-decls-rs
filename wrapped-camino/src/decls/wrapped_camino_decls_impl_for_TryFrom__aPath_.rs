use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Converts a [`Path`] to a [`Utf8Path`].
///
/// Returns [`FromPathError`] if the path is not valid UTF-8.
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
/// use std::convert::TryFrom;
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
/// use std::path::Path;
///
/// let unicode_path = Path::new("/valid/unicode");
/// <&Utf8Path>::try_from(unicode_path).expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let non_unicode_path = Path::new(non_unicode_str);
/// # #[cfg(unix)]
/// assert!(<&Utf8Path>::try_from(non_unicode_path).is_err(), "non-Unicode path failed");
/// ```
impl<'a> TryFrom<&'a Path> for &'a Utf8Path {
    type Error = FromPathError;
    fn try_from(path: &'a Path) -> Result<&'a Utf8Path, Self::Error> {
        Utf8Path::from_path(path).ok_or(FromPathError(()))
    }
}
