// Generated macro for CompareResult (enum)
macro_rules! Depcrate_traitsCompareResult {
() => {
// Module: crate::traits
// Provides: {"CompareResult"}
// Dependencies: {}
# [doc = " Indicates whether a comparison was successful, an error, or"] # [doc = " if more data was needed"] # [derive (Debug , Eq , PartialEq)] pub enum CompareResult { # [doc = " Comparison was successful"] Ok , # [doc = " We need more data to be sure"] Incomplete , # [doc = " Comparison failed"] Error , }
};
}
