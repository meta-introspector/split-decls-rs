use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A possible error value while converting a [`PathBuf`] to a [`Utf8PathBuf`].
///
/// Produced by the [`TryFrom<&PathBuf>`][tryfrom] implementation for [`Utf8PathBuf`].
///
/// [tryfrom]: Utf8PathBuf#impl-TryFrom<PathBuf>-for-Utf8PathBuf
///
/// # Examples
///
/// ```
/// use camino::{Utf8PathBuf, FromPathBufError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
/// use std::path::PathBuf;
///
/// let unicode_path = PathBuf::from("/valid/unicode");
/// let utf8_path_buf: Utf8PathBuf = unicode_path.try_into().expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let non_unicode_path = PathBuf::from(non_unicode_str);
/// # #[cfg(unix)]
/// let err: FromPathBufError = Utf8PathBuf::try_from(non_unicode_path.clone())
///     .expect_err("non-Unicode path failed");
/// # #[cfg(unix)]
/// assert_eq!(err.as_path(), &non_unicode_path);
/// # #[cfg(unix)]
/// assert_eq!(err.into_path_buf(), non_unicode_path);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FromPathBufError {
    path: PathBuf,
    error: FromPathError,
}
