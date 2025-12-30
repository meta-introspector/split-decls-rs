// Generated macro for __fallback_if_not_set (macro)
macro_rules! Depcrate___macros_fallback__fallback_if_not_set {
() => {
// Module: crate::__macros::fallback
// Provides: {"__fallback_if_not_set"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __fallback_if_not_set { (() ($ ($ fallback : tt) *)) => { $ ($ fallback) * } ; (($ ($ actual : tt) +) ($ ($ _fallback : tt) *)) => { $ ($ actual) + } ; }
};
}
