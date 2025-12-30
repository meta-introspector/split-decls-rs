// Generated macro for hpack_fuzz (function)
macro_rules! Depcrate_hpack_test_fuzzhpack_fuzz {
() => {
// Module: crate::hpack::test::fuzz
// Provides: {"hpack_fuzz"}
// Dependencies: {}
# [test] fn hpack_fuzz () { let _ = env_logger :: try_init () ; fn prop (fuzz : FuzzHpack) -> TestResult { fuzz . run () ; TestResult :: from_bool (true) } QuickCheck :: new () . tests (100) . quickcheck (prop as fn (FuzzHpack) -> TestResult) }
};
}
