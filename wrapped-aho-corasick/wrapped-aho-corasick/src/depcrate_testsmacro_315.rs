// Generated macro for macro_315 (macro)
macro_rules! Depcrate_testsmacro_315 {
() => {
// Module: crate::tests
// Provides: {"macro_315"}
// Dependencies: {}
testconfig ! (anchored , search_leftmost_longest_anchored_dfa_default , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
};
}
