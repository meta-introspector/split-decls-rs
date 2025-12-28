macro_rules! deps {
    () => {
        Mmap!();
        MmapMut!();
        RemapOptions!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (target_os = "linux")] impl RemapOptions { # [doc = " Creates a mew set of options for resizing a memory map."] pub fn new () -> Self { Self :: default () } # [doc = " Controls whether the memory map can be moved if it is not possible to"] # [doc = " resize it in place."] # [doc = ""] # [doc = " If false then the memory map is guaranteed to remain at the same"] # [doc = " address when being resized but attempting to resize will return an"] # [doc = " error if the new memory map would overlap with something else in the"] # [doc = " current process' memory."] # [doc = ""] # [doc = " By default this is false."] # [doc = ""] # [doc = " # `may_move` and `StableDeref`"] # [doc = " If the `stable_deref_trait` feature is enabled then [`Mmap`] and"] # [doc = " [`MmapMut`] implement `StableDeref`. `StableDeref` promises that the"] # [doc = " memory map dereferences to a fixed address, however, calling `remap`"] # [doc = " with `may_move` set may result in the backing memory of the mapping"] # [doc = " being moved to a new address. This may cause UB in other code"] # [doc = " depending on the `StableDeref` guarantees."] pub fn may_move (mut self , may_move : bool) -> Self { self . may_move = may_move ; self } pub (crate) fn into_flags (self) -> libc :: c_int { if self . may_move { libc :: MREMAP_MAYMOVE } else { 0 } } }
    };
}

impl_38!()