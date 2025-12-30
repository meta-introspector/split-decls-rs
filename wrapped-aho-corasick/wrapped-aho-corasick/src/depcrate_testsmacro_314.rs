// Generated macro for macro_314 (macro)
macro_rules! Depcrate_testsmacro_314 {
() => {
// Module: crate::tests
// Provides: {"macro_314"}
// Dependencies: {}
testconfig ! (anchored , search_leftmost_longest_anchored_nfa_contig_default , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
};
}
