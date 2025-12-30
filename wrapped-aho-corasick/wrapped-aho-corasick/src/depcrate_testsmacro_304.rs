// Generated macro for macro_304 (macro)
macro_rules! Depcrate_testsmacro_304 {
() => {
// Module: crate::tests
// Provides: {"macro_304"}
// Dependencies: {}
testconfig ! (anchored , search_standard_anchored_nfa_contig_default , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: ContiguousNFA)) ; }) ;
};
}
