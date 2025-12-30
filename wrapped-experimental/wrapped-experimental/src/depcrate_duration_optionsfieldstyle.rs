// Generated macro for FieldStyle (enum)
macro_rules! Depcrate_duration_optionsFieldStyle {
() => {
// Module: crate::duration::options
// Provides: {"FieldStyle"}
// Dependencies: {}
# [doc = " Enum used to process different unit styles in a generic way."] # [doc = " Implements `From` and `TryFrom` for all unit enums."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum FieldStyle { # [doc = " Narrow style (most compact)"] Narrow , # [doc = " Short style (default)"] Short , # [doc = " Long style (most verbose)"] Long , # [doc = " Ensure formatted value is at least two digits long (by appending leading zeroes, if necessary)"] TwoDigit , # [doc = " Numeric style"] Numeric , # [doc = " Fractional style"] Fractional , # [doc = " Digital style"] Digital , }
};
}
