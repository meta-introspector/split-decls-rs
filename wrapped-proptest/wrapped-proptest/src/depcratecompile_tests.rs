// Generated macro for compile_tests (function)
macro_rules! Depcratecompile_tests {
() => {
// Module: crate
// Provides: {"compile_tests"}
// Dependencies: {}
# [cfg (feature = "attr-macro")] # [test] fn compile_tests () { let t = trybuild :: TestCases :: new () ; t . pass ("tests/pass/*.rs") ; }
};
}
