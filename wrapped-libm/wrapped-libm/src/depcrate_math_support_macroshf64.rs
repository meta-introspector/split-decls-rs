// Generated macro for hf64 (macro)
macro_rules! Depcrate_math_support_macroshf64 {
() => {
// Module: crate::math::support::macros
// Provides: {"hf64"}
// Dependencies: {}
# [doc = " Construct a 64-bit float from hex float representation (C-style), guaranteed to"] # [doc = " evaluate at compile time."] # [allow (unused_macros)] # [cfg_attr (feature = "unstable-public-internals" , macro_export)] macro_rules ! hf64 { ($ s : literal) => { { const X : f64 = $ crate :: support :: hf64 ($ s) ; X } } ; }
};
}
