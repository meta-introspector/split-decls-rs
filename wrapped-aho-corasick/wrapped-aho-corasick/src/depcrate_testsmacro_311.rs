// Generated macro for macro_311 (macro)
macro_rules! Depcrate_testsmacro_311 {
() => {
// Module: crate::tests
// Provides: {"macro_311"}
// Dependencies: {}
testconfig ! (anchored , search_leftmost_first_anchored_dfa_start_both , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Both) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
};
}
