// Generated macro for slice_shl (function)
macro_rules! Depcrate_collections_btree_nodeslice_shl {
() => {
// Module: crate::collections::btree::node
// Provides: {"slice_shl"}
// Dependencies: {}
# [doc = " Shifts the elements in a slice `distance` positions to the left."] # [doc = ""] # [doc = " # Safety"] # [doc = " The slice has at least `distance` elements."] unsafe fn slice_shl < T > (slice : & mut [MaybeUninit < T >] , distance : usize) { unsafe { let slice_ptr = slice . as_mut_ptr () ; ptr :: copy (slice_ptr . add (distance) , slice_ptr , slice . len () - distance) ; } }
};
}
