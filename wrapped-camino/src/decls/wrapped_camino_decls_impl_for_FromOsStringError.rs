use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FromOsStringError {
    /// Returns the [`OsStr`] slice that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn as_os_str(&self) -> &OsStr {
        &self.os_string
    }
    /// Returns the [`OsString`] that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn into_os_string(self) -> OsString {
        self.os_string
    }
    /// Fetches a [`FromOsStrError`] for more about the conversion failure.
    ///
    /// At the moment this struct does not contain any additional information, but is provided for
    /// completeness.
    #[inline]
    pub fn from_os_str_error(&self) -> FromOsStrError {
        self.error
    }
    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidData`](io::ErrorKind::InvalidData).
    ///
    /// Many users of [`FromOsStringError`] will want to convert it into an [`io::Error`].
    /// This is a convenience method to do that.
    pub fn into_io_error(self) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}
