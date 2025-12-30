// Generated macro for proptest_to_ascii_uppercase (function)
macro_rules! Depcrate_testsproptest_to_ascii_uppercase {
() => {
// Module: crate::tests
// Provides: {"proptest_to_ascii_uppercase"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_to_ascii_uppercase (# [strategy (rand_unicode ())] control : String) { let compact = CompactString :: new (& control) ; let control = control . to_ascii_uppercase () ; let compact = compact . to_ascii_uppercase () ; prop_assert_eq ! (control , compact) ; }
};
}
