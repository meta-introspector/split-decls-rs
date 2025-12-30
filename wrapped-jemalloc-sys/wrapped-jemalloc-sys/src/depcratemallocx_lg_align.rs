// Generated macro for MALLOCX_LG_ALIGN (function)
macro_rules! DepcrateMALLOCX_LG_ALIGN {
() => {
// Module: crate
// Provides: {"MALLOCX_LG_ALIGN"}
// Dependencies: {}
# [doc = " Align the memory allocation to start at an address that is a"] # [doc = " multiple of `1 << la`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It does not validate that `la` is within the valid range."] # [inline] pub const fn MALLOCX_LG_ALIGN (la : usize) -> c_int { la as c_int }
};
}
