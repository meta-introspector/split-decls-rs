// Generated macro for SafetyProperty (enum)
macro_rules! Depcrate_rust_typeSafetyProperty {
() => {
// Module: crate::rust_type
// Provides: {"SafetyProperty"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Hash , Clone)] pub enum SafetyProperty { # [doc = " The type is unsafe in the selected position."] Unsafe { reasons : Vec < String > } , # [doc = " The safety of using the type in this position is unknown."] Unknown { reasons : Vec < String > } , # [doc = " The type is always safe in this position (and methods/functions using"] # [doc = " this is thus eligible for being automatically marked safe)."] Safe , }
};
}
