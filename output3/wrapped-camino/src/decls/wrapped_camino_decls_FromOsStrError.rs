use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A possible error value while converting a [`OsStr`] to a [`Utf8Path`].
///
/// Produced by the `TryFrom<&OsStr>` implementation for [`&Utf8Path`](Utf8Path).
///
///
/// # Examples
///
/// ```
/// use camino::{Utf8Path, FromOsStrError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
///
/// let unicode_str = OsStr::new("/valid/unicode");
/// let utf8_path: &Utf8Path = unicode_str.try_into().expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let err: FromOsStrError = <&Utf8Path>::try_from(non_unicode_str)
///     .expect_err("non-Unicode path failed");
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FromOsStrError(());
