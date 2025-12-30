// Generated macro for hf32 (macro)
macro_rules! Depcrate_math_support_macroshf32 {
() => {
// Module: crate::math::support::macros
// Provides: {"hf32"}
// Dependencies: {}
# [doc = " Construct a 32-bit float from hex float representation (C-style), guaranteed to"] # [doc = " evaluate at compile time."] # [allow (unused_macros)] # [cfg_attr (feature = "unstable-public-internals" , macro_export)] macro_rules ! hf32 { ($ s : literal) => { { const X : f32 = $ crate :: support :: hf32 ($ s) ; X } } ; }
};
}
