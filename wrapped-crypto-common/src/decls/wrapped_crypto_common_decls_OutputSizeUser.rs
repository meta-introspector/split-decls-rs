use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which return data with the given size.
pub trait OutputSizeUser {
    /// Size of the output in bytes.
    type OutputSize: ArraySize;
    /// Return output size in bytes.
    #[inline(always)]
    fn output_size() -> usize {
        Self::OutputSize::USIZE
    }
}
