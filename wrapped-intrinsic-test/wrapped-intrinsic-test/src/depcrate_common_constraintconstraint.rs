// Generated macro for Constraint (enum)
macro_rules! Depcrate_common_constraintConstraint {
() => {
// Module: crate::common::constraint
// Provides: {"Constraint"}
// Dependencies: {}
# [doc = " Describes the values to test for a const generic parameter."] # [derive (Debug , PartialEq , Clone , Deserialize)] pub enum Constraint { # [doc = " Test a single value."] Equal (i64) , # [doc = " Test a range of values, e.g. `0..16`."] Range (Range < i64 >) , # [doc = " Test discrete values, e.g. `vec![1, 2, 4, 8]`."] Set (Vec < i64 >) , }
};
}
