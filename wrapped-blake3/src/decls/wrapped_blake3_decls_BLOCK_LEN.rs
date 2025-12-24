use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The number of bytes in a block, 64.
///
/// You don't usually need to think about this number. One case where it matters is calling
/// [`OutputReader::fill`] in a loop, where using a `buf` argument that's a multiple of `BLOCK_LEN`
/// avoids repeating work.
pub const BLOCK_LEN: usize = 64;
