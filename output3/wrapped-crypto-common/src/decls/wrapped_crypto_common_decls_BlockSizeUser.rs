use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which process data in blocks.
pub trait BlockSizeUser {
    /// Size of the block in bytes.
    type BlockSize: BlockSizes;
    /// Return block size in bytes.
    #[inline(always)]
    fn block_size() -> usize {
        Self::BlockSize::USIZE
    }
}
