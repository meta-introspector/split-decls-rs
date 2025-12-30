// Generated macro for tests (module)
macro_rules! Depcrate_common_expecttests {
() => {
// Module: crate::common::expect
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: Expect ; # [test] fn expect_continue () { assert_eq ! (test_decode ::< Expect > (& ["100-continue"]) , Some (Expect :: CONTINUE) ,) ; } # [test] fn expectation_failed () { assert_eq ! (test_decode ::< Expect > (& ["sandwich"]) , None ,) ; } # [test] fn too_many_values () { assert_eq ! (test_decode ::< Expect > (& ["100-continue" , "100-continue"]) , None ,) ; } }
};
}
