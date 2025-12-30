// Generated macro for hf128 (macro)
macro_rules! Depcrate_math_support_macroshf128 {
() => {
// Module: crate::math::support::macros
// Provides: {"hf128"}
// Dependencies: {}
# [doc = " Construct a 128-bit float from hex float representation (C-style), guaranteed to"] # [doc = " evaluate at compile time."] # [cfg (f128_enabled)] # [allow (unused_macros)] # [cfg_attr (feature = "unstable-public-internals" , macro_export)] macro_rules ! hf128 { ($ s : literal) => { { const X : f128 = $ crate :: support :: hf128 ($ s) ; X } } ; }
};
}
