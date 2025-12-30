// Generated macro for macro_309 (macro)
macro_rules! Depcrate_testsmacro_309 {
() => {
// Module: crate::tests
// Provides: {"macro_309"}
// Dependencies: {}
testconfig ! (anchored , search_leftmost_first_anchored_nfa_contig_default , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
};
}
