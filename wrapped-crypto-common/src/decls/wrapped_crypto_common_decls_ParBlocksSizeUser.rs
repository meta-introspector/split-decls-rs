use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which can process blocks in parallel.
pub trait ParBlocksSizeUser: BlockSizeUser {
    /// Number of blocks which can be processed in parallel.
    type ParBlocksSize: ArraySize;
}
