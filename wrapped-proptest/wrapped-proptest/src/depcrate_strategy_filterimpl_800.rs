// Generated macro for impl_800 (impl)
macro_rules! Depcrate_strategy_filterimpl_800 {
() => {
// Module: crate::strategy::filter
// Provides: {"impl_800"}
// Dependencies: {}
impl < S : ValueTree , F : Fn (& S :: Value) -> bool > Filter < S , F > { fn ensure_acceptable (& mut self) { while ! (self . fun) (& self . source . current ()) { if ! self . source . complicate () { panic ! ("Unable to complicate filtered strategy \
                     back into acceptable value") ; } } } }
};
}
