// Generated macro for impl_283 (impl)
macro_rules! Depcrate_hirimpl_283 {
() => {
// Module: crate::hir
// Provides: {"impl_283"}
// Dependencies: {}
impl Repetition { # [doc = " Returns a new repetition with the same `min`, `max` and `greedy`"] # [doc = " values, but with its sub-expression replaced with the one given."] pub fn with (& self , sub : Hir) -> Repetition { Repetition { min : self . min , max : self . max , greedy : self . greedy , sub : Box :: new (sub) , } } }
};
}
