// Generated macro for ToolState (enum)
macro_rules! Depcrate_core_build_steps_toolstateToolState {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"ToolState"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , Deserialize , Serialize , PartialEq , PartialOrd)] # [serde (rename_all = "kebab-case")] # [doc = " Whether a tool can be compiled, tested or neither"] pub enum ToolState { # [doc = " The tool compiles successfully, but the test suite fails"] TestFail = 1 , # [doc = " The tool compiles successfully and its test suite passes"] TestPass = 2 , # [doc = " The tool can't even be compiled"] BuildFail = 0 , }
};
}
