// Generated macro for test_failing_compilation (function)
macro_rules! Depcratetest_failing_compilation {
() => {
// Module: crate
// Provides: {"test_failing_compilation"}
// Dependencies: {}
# [rustversion :: stable] # [test] fn test_failing_compilation () { let t = trybuild :: TestCases :: new () ; t . compile_fail ("fail/**/*.rs") ; }
};
}
