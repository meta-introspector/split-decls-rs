// Generated macro for release_shared (function)
macro_rules! Depcrate_bytesrelease_shared {
() => {
// Module: crate::bytes
// Provides: {"release_shared"}
// Dependencies: {}
unsafe fn release_shared (ptr : * mut Shared) { if (* ptr) . ref_cnt . fetch_sub (1 , Ordering :: Release) != 1 { return ; } (* ptr) . ref_cnt . load (Ordering :: Acquire) ; drop (Box :: from_raw (ptr)) ; }
};
}
