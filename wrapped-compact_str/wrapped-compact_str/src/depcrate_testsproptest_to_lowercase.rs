// Generated macro for proptest_to_lowercase (function)
macro_rules! Depcrate_testsproptest_to_lowercase {
() => {
// Module: crate::tests
// Provides: {"proptest_to_lowercase"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_to_lowercase (# [strategy (rand_unicode ())] control : String) { let compact = CompactString :: new (& control) ; let control = control . to_lowercase () ; let compact = compact . to_lowercase () ; prop_assert_eq ! (control , compact) ; }
};
}
