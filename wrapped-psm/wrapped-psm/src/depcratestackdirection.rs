// Generated macro for StackDirection (enum)
macro_rules! DepcrateStackDirection {
() => {
// Module: crate
// Provides: {"StackDirection"}
// Dependencies: {}
# [doc = " The direction into which stack grows as stack frames are made."] # [doc = ""] # [doc = " This is a target-specific property that can be obtained at runtime by calling"] # [doc = " `StackDirection::new()`."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub enum StackDirection { Ascending = 1 , Descending = 2 , }
};
}
