use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parallel blocks on which [`ParBlocksSizeUser`] implementors operate.
pub type ParBlocks<T> = Array<Block<T>, <T as ParBlocksSizeUser>::ParBlocksSize>;
