// Generated macro for macro_318 (macro)
macro_rules! Depcrate_testsmacro_318 {
() => {
// Module: crate::tests
// Provides: {"macro_318"}
// Dependencies: {}
testconfig ! (acasei_standard_nfa_noncontig_default , & [ASCII_CASE_INSENSITIVE] , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . prefilter (false) . ascii_case_insensitive (true) ; }) ;
};
}
