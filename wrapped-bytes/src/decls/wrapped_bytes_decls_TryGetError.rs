use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error type for the `try_get_` methods of [`Buf`].
/// Indicates that there were not enough remaining
/// bytes in the buffer while attempting
/// to get a value from a [`Buf`] with one
/// of the `try_get_` methods.
#[derive(Debug, PartialEq, Eq)]
pub struct TryGetError {
    /// The number of bytes necessary to get the value
    pub requested: usize,
    /// The number of bytes available in the buffer
    pub available: usize,
}
