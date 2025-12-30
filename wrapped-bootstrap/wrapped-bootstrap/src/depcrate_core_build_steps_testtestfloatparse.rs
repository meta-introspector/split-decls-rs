// Generated macro for TestFloatParse (struct)
macro_rules! Depcrate_core_build_steps_testTestFloatParse {
() => {
// Module: crate::core::build_steps::test
// Provides: {"TestFloatParse"}
// Dependencies: {}
# [doc = " Test step that does two things:"] # [doc = " - Runs `cargo test` for the `src/tools/test-float-parse` tool."] # [doc = " - Invokes the `test-float-parse` tool to test the standard library's"] # [doc = "   float parsing routines."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct TestFloatParse { # [doc = " The build compiler which will build and run unit tests of `test-float-parse`, and which will"] # [doc = " build the `test-float-parse` tool itself."] # [doc = ""] # [doc = " Note that the staging is a bit funny here, because this step essentially tests std, but it"] # [doc = " also needs to build the tool. So if we test stage1 std, we build:"] # [doc = " 1) stage1 rustc"] # [doc = " 2) Use that to build stage1 libstd"] # [doc = " 3) Use that to build and run *stage2* test-float-parse"] build_compiler : Compiler , # [doc = " Target for which we build std and test that std."] target : TargetSelection , }
};
}
