// Generated macro for vptr (function)
macro_rules! Depcrate_bytes_mutvptr {
() => {
// Module: crate::bytes_mut
// Provides: {"vptr"}
// Dependencies: {}
# [inline] fn vptr (ptr : * mut u8) -> NonNull < u8 > { if cfg ! (debug_assertions) { NonNull :: new (ptr) . expect ("Vec pointer should be non-null") } else { unsafe { NonNull :: new_unchecked (ptr) } } }
};
}
