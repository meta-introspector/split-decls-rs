// Generated macro for get_runners (function)
macro_rules! Depcrate_testget_runners {
() => {
// Module: crate::test
// Provides: {"get_runners"}
// Dependencies: {}
fn get_runners () -> Runners { let mut runners = HashMap :: new () ; runners . insert ("--test-rustc" , ("Run all rustc tests" , test_rustc as Runner)) ; runners . insert ("--test-successful-rustc" , ("Run successful rustc tests" , test_successful_rustc)) ; runners . insert ("--test-failing-ui-pattern-tests" , ("Run failing ui pattern tests" , test_failing_ui_pattern_tests) ,) ; runners . insert ("--test-failing-rustc" , ("Run failing rustc tests" , test_failing_rustc)) ; runners . insert ("--projects" , ("Run the tests of popular crates" , test_projects)) ; runners . insert ("--test-libcore" , ("Run libcore tests" , test_libcore)) ; runners . insert ("--clean" , ("Empty cargo target directory" , clean)) ; runners . insert ("--build-sysroot" , ("Build sysroot" , build_sysroot)) ; runners . insert ("--std-tests" , ("Run std tests" , std_tests)) ; runners . insert ("--asm-tests" , ("Run asm tests" , asm_tests)) ; runners . insert ("--extended-tests" , ("Run extended sysroot tests" , extended_sysroot_tests)) ; runners . insert ("--extended-rand-tests" , ("Run extended rand tests" , extended_rand_tests)) ; runners . insert ("--extended-regex-example-tests" , ("Run extended regex example tests" , extended_regex_example_tests) ,) ; runners . insert ("--extended-regex-tests" , ("Run extended regex tests" , extended_regex_tests)) ; runners . insert ("--mini-tests" , ("Run mini tests" , mini_tests)) ; runners . insert ("--cargo-tests" , ("Run cargo tests" , cargo_tests)) ; runners }
};
}
