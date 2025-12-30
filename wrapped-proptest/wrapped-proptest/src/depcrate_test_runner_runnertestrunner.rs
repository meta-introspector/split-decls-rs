// Generated macro for TestRunner (struct)
macro_rules! Depcrate_test_runner_runnerTestRunner {
() => {
// Module: crate::test_runner::runner
// Provides: {"TestRunner"}
// Dependencies: {}
# [doc = " State used when running a proptest test."] # [derive (Clone)] pub struct TestRunner { config : Config , successes : u32 , local_rejects : u32 , global_rejects : u32 , rng : TestRng , flat_map_regens : Arc < AtomicUsize > , local_reject_detail : RejectionDetail , global_reject_detail : RejectionDetail , }
};
}
