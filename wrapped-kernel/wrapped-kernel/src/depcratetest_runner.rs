// Generated macro for test_runner (function)
macro_rules! Depcratetest_runner {
() => {
// Module: crate
// Provides: {"test_runner"}
// Dependencies: {}
# [cfg (test)] pub fn test_runner (tests : & [& dyn Fn ()]) { println ! ("Running {} tests" , tests . len ()) ; for test in tests { test () ; } core_scheduler () . exit (0) }
};
}
