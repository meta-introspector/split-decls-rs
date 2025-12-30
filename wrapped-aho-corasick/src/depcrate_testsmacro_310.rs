// Generated macro for macro_310 (macro)
macro_rules! Depcrate_testsmacro_310 {
() => {
// Module: crate::tests
// Provides: {"macro_310"}
// Dependencies: {}
testconfig ! (anchored , search_leftmost_first_anchored_dfa_default , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
};
}
