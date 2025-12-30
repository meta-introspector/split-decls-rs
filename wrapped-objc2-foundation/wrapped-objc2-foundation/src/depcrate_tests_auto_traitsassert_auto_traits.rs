// Generated macro for assert_auto_traits (function)
macro_rules! Depcrate_tests_auto_traitsassert_auto_traits {
() => {
// Module: crate::tests::auto_traits
// Provides: {"assert_auto_traits"}
// Dependencies: {}
fn assert_auto_traits < T : Send + Sync + UnwindSafe + RefUnwindSafe > () { assert_unwindsafe :: < T > () ; }
};
}
