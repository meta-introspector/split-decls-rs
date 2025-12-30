// Generated macro for hf16 (macro)
macro_rules! Depcrate_math_support_macroshf16 {
() => {
// Module: crate::math::support::macros
// Provides: {"hf16"}
// Dependencies: {}
# [doc = " Construct a 16-bit float from hex float representation (C-style), guaranteed to"] # [doc = " evaluate at compile time."] # [cfg (f16_enabled)] # [cfg_attr (feature = "unstable-public-internals" , macro_export)] # [allow (unused_macros)] macro_rules ! hf16 { ($ s : literal) => { { const X : f16 = $ crate :: support :: hf16 ($ s) ; X } } ; }
};
}
