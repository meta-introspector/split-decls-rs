// Generated macro for slice_shr (function)
macro_rules! Depcrate_collections_btree_nodeslice_shr {
() => {
// Module: crate::collections::btree::node
// Provides: {"slice_shr"}
// Dependencies: {}
# [doc = " Shifts the elements in a slice `distance` positions to the right."] # [doc = ""] # [doc = " # Safety"] # [doc = " The slice has at least `distance` elements."] unsafe fn slice_shr < T > (slice : & mut [MaybeUninit < T >] , distance : usize) { unsafe { let slice_ptr = slice . as_mut_ptr () ; ptr :: copy (slice_ptr , slice_ptr . add (distance) , slice . len () - distance) ; } }
};
}
