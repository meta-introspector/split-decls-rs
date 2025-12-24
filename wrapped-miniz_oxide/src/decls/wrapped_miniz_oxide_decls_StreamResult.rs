use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A structure containing the result of a call to the inflate or deflate streaming functions.
#[cfg(not(feature = "rustc-dep-of-std"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StreamResult {
    /// The number of bytes consumed from the input slice.
    pub bytes_consumed: usize,
    /// The number of bytes written to the output slice.
    pub bytes_written: usize,
    /// The return status of the call.
    pub status: MZResult,
}
