// Generated macro for MALLOCX_TCACHE_NONE (const)
macro_rules! DepcrateMALLOCX_TCACHE_NONE {
() => {
// Module: crate
// Provides: {"MALLOCX_TCACHE_NONE"}
// Dependencies: {}
# [doc = " Do not use a thread-specific cache (_tcache_)."] # [doc = ""] # [doc = " Unless `MALLOCX_TCACHE(tc)` or `MALLOCX_TCACHE_NONE` is specified, an"] # [doc = " automatically managed _tcache_ will be used under many circumstances."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This option cannot be used in the same `flags` argument as"] # [doc = " `MALLOCX_TCACHE(tc)`."] pub const MALLOCX_TCACHE_NONE : c_int = MALLOCX_TCACHE ((- 1isize) as usize) ;
};
}
