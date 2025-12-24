use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A possible error value while converting a [`Path`] to a [`Utf8Path`].
///
/// Produced by the [`TryFrom<&Path>`][tryfrom] implementation for [`&Utf8Path`](Utf8Path).
///
/// [tryfrom]: Utf8Path#impl-TryFrom<%26Path>-for-%26Utf8Path
///
///
/// # Examples
///
/// ```
/// use camino::{Utf8Path, FromPathError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
/// use std::path::Path;
///
/// let unicode_path = Path::new("/valid/unicode");
/// let utf8_path: &Utf8Path = unicode_path.try_into().expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let non_unicode_path = Path::new(non_unicode_str);
/// # #[cfg(unix)]
/// let err: FromPathError = <&Utf8Path>::try_from(non_unicode_path)
///     .expect_err("non-Unicode path failed");
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FromPathError(());
