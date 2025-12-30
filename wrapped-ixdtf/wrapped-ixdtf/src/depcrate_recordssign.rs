// Generated macro for Sign (enum)
macro_rules! Depcrate_recordsSign {
() => {
// Module: crate::records
// Provides: {"Sign"}
// Dependencies: {}
# [doc = " The parsed sign value, representing whether its struct is positive or negative."] # [repr (i8)] # [allow (clippy :: exhaustive_enums)] # [derive (Debug , Clone , Copy , PartialEq)] pub enum Sign { # [doc = " A negative value sign, representable as either -1 or false."] Negative = - 1 , # [doc = " A positive value sign, representable as either 1 or true."] Positive = 1 , }
};
}
