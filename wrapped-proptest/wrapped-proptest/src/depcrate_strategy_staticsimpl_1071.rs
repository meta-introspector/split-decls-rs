// Generated macro for impl_1071 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1071 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1071"}
// Dependencies: {}
impl < S : ValueTree , F : FilterFn < S :: Value > > Filter < S , F > { fn ensure_acceptable (& mut self) { while ! self . fun . apply (& self . source . current ()) { if ! self . source . complicate () { panic ! ("Unable to complicate filtered strategy \
                     back into acceptable value") ; } } } }
};
}
