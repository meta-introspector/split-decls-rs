// Generated macro for non_fuzz_debug_assert (macro)
macro_rules! Depcrate_memnon_fuzz_debug_assert {
() => {
// Module: crate::mem
// Provides: {"non_fuzz_debug_assert"}
// Dependencies: {}
macro_rules ! non_fuzz_debug_assert { ($ ($ arg : tt) *) => (if ! cfg ! (fuzzing) { debug_assert ! ($ ($ arg) *) ; }) }
};
}
