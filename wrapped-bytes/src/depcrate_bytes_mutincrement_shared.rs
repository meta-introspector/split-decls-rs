// Generated macro for increment_shared (function)
macro_rules! Depcrate_bytes_mutincrement_shared {
() => {
// Module: crate::bytes_mut
// Provides: {"increment_shared"}
// Dependencies: {}
unsafe fn increment_shared (ptr : * mut Shared) { let old_size = (* ptr) . ref_count . fetch_add (1 , Ordering :: Relaxed) ; if old_size > isize :: MAX as usize { crate :: abort () ; } }
};
}
