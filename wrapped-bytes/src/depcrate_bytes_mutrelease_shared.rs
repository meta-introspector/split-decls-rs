// Generated macro for release_shared (function)
macro_rules! Depcrate_bytes_mutrelease_shared {
() => {
// Module: crate::bytes_mut
// Provides: {"release_shared"}
// Dependencies: {}
unsafe fn release_shared (ptr : * mut Shared) { if (* ptr) . ref_count . fetch_sub (1 , Ordering :: Release) != 1 { return ; } (* ptr) . ref_count . load (Ordering :: Acquire) ; drop (Box :: from_raw (ptr)) ; }
};
}
