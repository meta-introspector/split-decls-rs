// Generated macro for Compiletest (struct)
macro_rules! Depcrate_core_build_steps_testCompiletest {
() => {
// Module: crate::core::build_steps::test
// Provides: {"Compiletest"}
// Dependencies: {}
# [doc = " Executes the `compiletest` tool to run a suite of tests."] # [doc = ""] # [doc = " Compiles all tests with `test_compiler` for `target` with the specified"] # [doc = " compiletest `mode` and `suite` arguments. For example `mode` can be"] # [doc = " \"mir-opt\" and `suite` can be something like \"debuginfo\"."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] struct Compiletest { # [doc = " The compiler that we're testing."] test_compiler : Compiler , target : TargetSelection , mode : & 'static str , suite : & 'static str , path : & 'static str , compare_mode : Option < & 'static str > , }
};
}
