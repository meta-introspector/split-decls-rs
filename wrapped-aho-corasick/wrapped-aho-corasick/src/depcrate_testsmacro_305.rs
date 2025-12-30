// Generated macro for macro_305 (macro)
macro_rules! Depcrate_testsmacro_305 {
() => {
// Module: crate::tests
// Provides: {"macro_305"}
// Dependencies: {}
testconfig ! (anchored , search_standard_anchored_dfa_default , AC_STANDARD_ANCHORED_NON_OVERLAPPING , Standard , | b : & mut AhoCorasickBuilder | { b . start_kind (StartKind :: Anchored) . kind (Some (AhoCorasickKind :: DFA)) ; }) ;
};
}
