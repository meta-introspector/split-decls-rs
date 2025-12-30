// Generated macro for macro_295 (macro)
macro_rules! Depcrate_testsmacro_295 {
() => {
// Module: crate::tests
// Provides: {"macro_295"}
// Dependencies: {}
testconfig ! (overlapping , search_standard_overlapping_dfa_start_both_no_prefilter , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . start_kind (StartKind :: Both) . prefilter (false) ; }) ;
};
}
