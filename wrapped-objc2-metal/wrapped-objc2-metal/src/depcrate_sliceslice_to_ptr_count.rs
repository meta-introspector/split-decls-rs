// Generated macro for slice_to_ptr_count (function)
macro_rules! Depcrate_sliceslice_to_ptr_count {
() => {
// Module: crate::slice
// Provides: {"slice_to_ptr_count"}
// Dependencies: {}
# [allow (dead_code)] fn slice_to_ptr_count < T > (slice : & [T]) -> (NonNull < T > , usize) { let ptr : * const T = slice . as_ptr () ; let ptr : * mut T = ptr as * mut T ; let ptr = unsafe { NonNull :: new_unchecked (ptr) } ; (ptr , slice . len ()) }
};
}
