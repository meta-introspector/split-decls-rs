// Generated macro for MALLOCX_ALIGN (function)
macro_rules! DepcrateMALLOCX_ALIGN {
() => {
// Module: crate
// Provides: {"MALLOCX_ALIGN"}
// Dependencies: {}
# [doc = " Align the memory allocation to start at an address that is a multiple of `align`,"] # [doc = " where a is a power of two."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This macro does not validate that a is a power of 2."] # [inline] pub const fn MALLOCX_ALIGN (aling : usize) -> c_int { aling . trailing_zeros () as c_int }
};
}
