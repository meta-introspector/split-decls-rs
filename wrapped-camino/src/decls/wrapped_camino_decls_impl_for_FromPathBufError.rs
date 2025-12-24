use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FromPathBufError {
    /// Returns the [`Path`] slice that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn as_path(&self) -> &Path {
        &self.path
    }
    /// Returns the [`PathBuf`] that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn into_path_buf(self) -> PathBuf {
        self.path
    }
    /// Fetches a [`FromPathError`] for more about the conversion failure.
    ///
    /// At the moment this struct does not contain any additional information, but is provided for
    /// completeness.
    #[inline]
    pub fn from_path_error(&self) -> FromPathError {
        self.error
    }
    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidData`](io::ErrorKind::InvalidData).
    ///
    /// Many users of [`FromPathBufError`] will want to convert it into an [`io::Error`]. This is a
    /// convenience method to do that.
    pub fn into_io_error(self) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}
