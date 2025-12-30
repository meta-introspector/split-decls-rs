// Generated macro for TestFailed (struct)
macro_rules! Depcrate_format_testTestFailed {
() => {
// Module: crate::format::test
// Provides: {"TestFailed"}
// Dependencies: {}
# [doc = " Case-finished with failure event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub struct TestFailed { # [doc = " Test case name."] pub name : String , # [doc = " Test's stdout"] pub stdout : Option < String > , # [doc = " Test failure mssage"] pub message : Option < String > , }
};
}
