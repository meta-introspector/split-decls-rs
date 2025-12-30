// Generated macro for impl_270 (impl)
macro_rules! Depcrate_hybrid_dfaimpl_270 {
() => {
// Module: crate::hybrid::dfa
// Provides: {"impl_270"}
// Dependencies: {}
impl SearchProgress { # [doc = " Returns the length, in bytes, of this search so far."] # [doc = ""] # [doc = " This automatically handles the case of a reverse search, where `at`"] # [doc = " is likely to be less than `start`."] fn len (& self) -> usize { if self . start <= self . at { self . at - self . start } else { self . start - self . at } } }
};
}
