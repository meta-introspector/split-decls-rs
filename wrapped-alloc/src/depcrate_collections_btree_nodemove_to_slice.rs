// Generated macro for move_to_slice (function)
macro_rules! Depcrate_collections_btree_nodemove_to_slice {
() => {
// Module: crate::collections::btree::node
// Provides: {"move_to_slice"}
// Dependencies: {}
# [doc = " Moves all values from a slice of initialized elements to a slice"] # [doc = " of uninitialized elements, leaving behind `src` as all uninitialized."] # [doc = " Works like `dst.copy_from_slice(src)` but does not require `T` to be `Copy`."] fn move_to_slice < T > (src : & mut [MaybeUninit < T >] , dst : & mut [MaybeUninit < T >]) { assert ! (src . len () == dst . len ()) ; unsafe { ptr :: copy_nonoverlapping (src . as_ptr () , dst . as_mut_ptr () , src . len ()) ; } }
};
}
