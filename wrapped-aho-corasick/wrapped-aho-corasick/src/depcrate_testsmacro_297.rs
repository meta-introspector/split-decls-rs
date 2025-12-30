// Generated macro for macro_297 (macro)
macro_rules! Depcrate_testsmacro_297 {
() => {
// Module: crate::tests
// Provides: {"macro_297"}
// Dependencies: {}
testconfig ! (overlapping , search_standard_overlapping_dfa_start_both_no_byte_class , AC_STANDARD_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . kind (Some (AhoCorasickKind :: DFA)) . start_kind (StartKind :: Both) . byte_classes (false) ; }) ;
};
}
