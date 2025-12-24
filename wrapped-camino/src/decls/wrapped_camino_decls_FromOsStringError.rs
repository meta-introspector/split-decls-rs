use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A possible error value while converting a [`OsString`] to a [`Utf8PathBuf`].
///
/// Produced by the `TryFrom<OsString>` implementation for [`Utf8PathBuf`].
///
/// # Examples
///
/// ```
/// # #[cfg(osstring_from_str)] {
/// use camino::{Utf8PathBuf, FromOsStringError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// use std::str::FromStr;
/// use std::ffi::OsString;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
///
/// let unicode_string = OsString::from_str("/valid/unicode").unwrap();
/// let utf8_path_buf: Utf8PathBuf = unicode_string.try_into()
///     .expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_string = OsStr::from_bytes(b"\xFF\xFF\xFF").to_owned();
/// # #[cfg(unix)]
/// let err: FromOsStringError = Utf8PathBuf::try_from(non_unicode_string.clone())
///     .expect_err("non-Unicode path failed");
/// # #[cfg(unix)]
/// assert_eq!(err.as_os_str(), &non_unicode_string);
/// # #[cfg(unix)]
/// assert_eq!(err.into_os_string(), non_unicode_string);
/// # }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FromOsStringError {
    os_string: OsString,
    error: FromOsStrError,
}
