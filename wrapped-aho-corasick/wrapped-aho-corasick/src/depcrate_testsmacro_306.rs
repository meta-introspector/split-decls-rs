// Generated macro for macro_306 (macro)
macro_rules! Depcrate_testsmacro_306 {
() => {
// Module: crate::tests
// Provides: {"macro_306"}
// Dependencies: {}
testconfig ! (anchored , search_standard_anchored_dfa_start_both , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Both) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
};
}
