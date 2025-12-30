// Generated macro for DrainFilter (struct)
macro_rules! Depcrate_collections_vecDrainFilter {
() => {
// Module: crate::collections::vec
// Provides: {"DrainFilter"}
// Dependencies: {}
# [doc = " An iterator produced by calling [`Vec::drain_filter`]."] # [derive (Debug)] pub struct DrainFilter < 'a , 'bump : 'a , T : 'a + 'bump , F > where F : FnMut (& mut T) -> bool , { vec : & 'a mut Vec < 'bump , T > , idx : usize , del : usize , old_len : usize , pred : F , }
};
}
