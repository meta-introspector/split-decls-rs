use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FromOsStrError {
    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidData`](io::ErrorKind::InvalidData).
    ///
    /// Many users of [`FromOsStrError`] will want to convert it into an [`io::Error`]. This is a
    /// convenience method to do that.
    pub fn into_io_error(self) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}
