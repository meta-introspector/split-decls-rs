// Generated macro for RangeError (struct)
macro_rules! Depcrate_errorRangeError {
() => {
// Module: crate::error
// Provides: {"RangeError"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Display)] # [doc = " An argument is out of range for its domain."] # [displaydoc ("The {field} = {value} argument is out of range {min}..={max}")] # [allow (clippy :: exhaustive_structs)] pub struct RangeError { # [doc = " The argument that is out of range, such as \"year\""] pub field : & 'static str , # [doc = " The actual value"] pub value : i32 , # [doc = " The minimum value (inclusive). This might not be tight."] pub min : i32 , # [doc = " The maximum value (inclusive). This might not be tight."] pub max : i32 , }
};
}
