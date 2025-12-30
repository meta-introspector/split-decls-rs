// Generated macro for MALLOCX_TCACHE (function)
macro_rules! DepcrateMALLOCX_TCACHE {
() => {
// Module: crate
// Provides: {"MALLOCX_TCACHE"}
// Dependencies: {}
# [doc = " Use the thread-specific cache (_tcache_) specified by the identifier `tc`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `tc` must have been acquired via the `tcache.create mallctl`. This function"] # [doc = " does not validate that `tc` specifies a valid identifier."] # [inline] pub const fn MALLOCX_TCACHE (tc : usize) -> c_int { tc . wrapping_add (2) . wrapping_shl (8) as c_int }
};
}
