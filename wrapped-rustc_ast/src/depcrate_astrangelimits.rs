// Generated macro for RangeLimits (enum)
macro_rules! Depcrate_astRangeLimits {
() => {
// Module: crate::ast
// Provides: {"RangeLimits"}
// Dependencies: {}
# [doc = " Limit types of a range (inclusive or exclusive)."] # [derive (Copy , Clone , PartialEq , Encodable , Decodable , Debug , Walkable)] pub enum RangeLimits { # [doc = " Inclusive at the beginning, exclusive at the end."] HalfOpen , # [doc = " Inclusive at the beginning and end."] Closed , }
};
}
