// Generated macro for testable_result (function)
macro_rules! Depcrate_teststestable_result {
() => {
// Module: crate::tests
// Provides: {"testable_result"}
// Dependencies: {}
# [test] fn testable_result () { fn result () -> Result < bool , String > { Ok (true) } quickcheck (result as fn () -> Result < bool , String >) ; }
};
}
