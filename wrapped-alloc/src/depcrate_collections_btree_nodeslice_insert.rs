// Generated macro for slice_insert (function)
macro_rules! Depcrate_collections_btree_nodeslice_insert {
() => {
// Module: crate::collections::btree::node
// Provides: {"slice_insert"}
// Dependencies: {}
# [doc = " Inserts a value into a slice of initialized elements followed by one uninitialized element."] # [doc = ""] # [doc = " # Safety"] # [doc = " The slice has more than `idx` elements."] unsafe fn slice_insert < T > (slice : & mut [MaybeUninit < T >] , idx : usize , val : T) { unsafe { let len = slice . len () ; debug_assert ! (len > idx) ; let slice_ptr = slice . as_mut_ptr () ; if len > idx + 1 { ptr :: copy (slice_ptr . add (idx) , slice_ptr . add (idx + 1) , len - idx - 1) ; } (* slice_ptr . add (idx)) . write (val) ; } }
};
}
