// Generated macro for slice_remove (function)
macro_rules! Depcrate_collections_btree_nodeslice_remove {
() => {
// Module: crate::collections::btree::node
// Provides: {"slice_remove"}
// Dependencies: {}
# [doc = " Removes and returns a value from a slice of all initialized elements, leaving behind one"] # [doc = " trailing uninitialized element."] # [doc = ""] # [doc = " # Safety"] # [doc = " The slice has more than `idx` elements."] unsafe fn slice_remove < T > (slice : & mut [MaybeUninit < T >] , idx : usize) -> T { unsafe { let len = slice . len () ; debug_assert ! (idx < len) ; let slice_ptr = slice . as_mut_ptr () ; let ret = (* slice_ptr . add (idx)) . assume_init_read () ; ptr :: copy (slice_ptr . add (idx + 1) , slice_ptr . add (idx) , len - idx - 1) ; ret } }
};
}
