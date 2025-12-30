// Generated macro for macro_319 (macro)
macro_rules! Depcrate_testsmacro_319 {
() => {
// Module: crate::tests
// Provides: {"macro_319"}
// Dependencies: {}
testconfig ! (acasei_standard_nfa_contig_default , & [ASCII_CASE_INSENSITIVE] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: ContiguousNFA)) . prefilter (false) . ascii_case_insensitive (true) ; }) ;
};
}
