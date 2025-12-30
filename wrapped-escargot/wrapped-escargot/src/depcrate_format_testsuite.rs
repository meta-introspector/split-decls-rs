// Generated macro for Suite (enum)
macro_rules! Depcrate_format_testSuite {
() => {
// Module: crate::format::test
// Provides: {"Suite"}
// Dependencies: {}
# [doc = " Suite event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [serde (rename_all = "snake_case")] # [serde (tag = "event")] pub enum Suite { # [doc = " Suite-started event."] Started (SuiteStarted) , # [doc = " Suite-finished successfully event."] Ok (SuiteOk) , # [doc = " Suite-finished with failure event."] Failed (SuiteFailed) , # [cfg (not (feature = "strict_unstable"))] # [doc (hidden)] # [serde (other)] Unknown , }
};
}
