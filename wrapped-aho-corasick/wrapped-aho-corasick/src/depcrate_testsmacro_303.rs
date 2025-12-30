// Generated macro for macro_303 (macro)
macro_rules! Depcrate_testsmacro_303 {
() => {
// Module: crate::tests
// Provides: {"macro_303"}
// Dependencies: {}
testconfig ! (anchored , search_standard_anchored_nfa_noncontig_default , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) ; }) ;
};
}
