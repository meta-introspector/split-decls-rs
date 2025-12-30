// Generated macro for TestOutcome (enum)
macro_rules! Depcrate_metricsTestOutcome {
() => {
// Module: crate::metrics
// Provides: {"TestOutcome"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , PartialEq , Eq , Hash)] # [serde (tag = "outcome" , rename_all = "snake_case")] pub enum TestOutcome { Passed , Failed , Ignored { ignore_reason : Option < String > } , }
};
}
