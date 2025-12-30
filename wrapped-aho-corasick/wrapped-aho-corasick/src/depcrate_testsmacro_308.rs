// Generated macro for macro_308 (macro)
macro_rules! Depcrate_testsmacro_308 {
() => {
// Module: crate::tests
// Provides: {"macro_308"}
// Dependencies: {}
testconfig ! (anchored , search_leftmost_first_anchored_nfa_noncontig_default , AC_LEFTMOST_FIRST_ANCHORED , LeftmostFirst , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
};
}
