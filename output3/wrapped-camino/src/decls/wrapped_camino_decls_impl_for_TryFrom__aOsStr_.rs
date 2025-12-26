use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Converts an [`OsStr`] to a [`Utf8Path`].
///
/// Returns the original [`OsStr`] if it is not valid UTF-8.
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
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// assert!(<&Utf8Path>::try_from(non_unicode_str).is_err(), "non-Unicode string path failed");
/// ```
impl<'a> TryFrom<&'a OsStr> for &'a Utf8Path {
    type Error = FromOsStrError;
    fn try_from(os_str: &'a OsStr) -> Result<&'a Utf8Path, Self::Error> {
        Utf8Path::from_os_str(os_str).ok_or(FromOsStrError(()))
    }
}
