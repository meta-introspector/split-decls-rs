// Generated macro for MALLOCX_ZERO (const)
macro_rules! DepcrateMALLOCX_ZERO {
() => {
// Module: crate
// Provides: {"MALLOCX_ZERO"}
// Dependencies: {}
# [doc = " Initialize newly allocated memory to contain zero bytes."] # [doc = ""] # [doc = " In the growing reallocation case, the real size prior to reallocation"] # [doc = " defines the boundary between untouched bytes and those that are initialized"] # [doc = " to contain zero bytes."] # [doc = ""] # [doc = " If this option is not set, newly allocated memory is uninitialized."] pub const MALLOCX_ZERO : c_int = 0x40 ;
};
}
