use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An arena that can hold objects of multiple different types that impl `Copy`
/// and/or satisfy `!mem::needs_drop`.
pub struct DroplessArena {
    /// A pointer to the start of the free space.
    start: Cell<*mut u8>,
    /// A pointer to the end of free space.
    ///
    /// The allocation proceeds downwards from the end of the chunk towards the
    /// start. (This is slightly simpler and faster than allocating upwards,
    /// see <https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html>.)
    /// When this pointer crosses the start pointer, a new chunk is allocated.
    ///
    /// This is kept aligned to DROPLESS_ALIGNMENT.
    end: Cell<*mut u8>,
    /// A vector of arena chunks.
    chunks: RefCell<Vec<ArenaChunk>>,
}
