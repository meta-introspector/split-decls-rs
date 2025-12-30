// Generated macro for CheckAction (enum)
macro_rules! Depcrate_precisionCheckAction {
() => {
// Module: crate::precision
// Provides: {"CheckAction"}
// Dependencies: {}
# [doc = " Result of checking for possible overrides."] # [derive (Debug , Default)] pub enum CheckAction { # [doc = " The check should pass. Default case."] # [default] AssertSuccess , # [doc = " Override the ULP for this check."] AssertWithUlp (u32) , # [doc = " Failure is expected, ensure this is the case (xfail). Takes a contxt string to help trace"] # [doc = " back exactly why we expect this to fail."] AssertFailure (& 'static str) , # [doc = " The override somehow validated the result, here it is."] Custom (TestResult) , # [doc = " Disregard the output."] Skip , }
};
}
