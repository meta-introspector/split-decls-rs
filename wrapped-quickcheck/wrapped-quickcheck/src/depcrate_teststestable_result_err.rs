// Generated macro for testable_result_err (function)
macro_rules! Depcrate_teststestable_result_err {
() => {
// Module: crate::tests
// Provides: {"testable_result_err"}
// Dependencies: {}
# [test] # [should_panic] fn testable_result_err () { quickcheck (Err :: < bool , i32 > as fn (i32) -> Result < bool , i32 >) ; }
};
}
