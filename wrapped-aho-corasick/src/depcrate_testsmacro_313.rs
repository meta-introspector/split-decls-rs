// Generated macro for macro_313 (macro)
macro_rules! Depcrate_testsmacro_313 {
() => {
// Module: crate::tests
// Provides: {"macro_313"}
// Dependencies: {}
testconfig ! (anchored , search_leftmost_longest_anchored_nfa_noncontig_default , AC_LEFTMOST_LONGEST_ANCHORED , LeftmostLongest , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
};
}
