use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_os = "linux")]
impl RemapOptions {
    /// Creates a mew set of options for resizing a memory map.
    pub fn new() -> Self {
        Self::default()
    }
    /// Controls whether the memory map can be moved if it is not possible to
    /// resize it in place.
    ///
    /// If false then the memory map is guaranteed to remain at the same
    /// address when being resized but attempting to resize will return an
    /// error if the new memory map would overlap with something else in the
    /// current process' memory.
    ///
    /// By default this is false.
    ///
    /// # `may_move` and `StableDeref`
    /// If the `stable_deref_trait` feature is enabled then [`Mmap`] and
    /// [`MmapMut`] implement `StableDeref`. `StableDeref` promises that the
    /// memory map dereferences to a fixed address, however, calling `remap`
    /// with `may_move` set may result in the backing memory of the mapping
    /// being moved to a new address. This may cause UB in other code
    /// depending on the `StableDeref` guarantees.
    pub fn may_move(mut self, may_move: bool) -> Self {
        self.may_move = may_move;
        self
    }
    pub(crate) fn into_flags(self) -> libc::c_int {
        if self.may_move {
            libc::MREMAP_MAYMOVE
        } else {
            0
        }
    }
}
