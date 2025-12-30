// Generated macro for InvalidValue (enum)
macro_rules! Depcrate_errorsInvalidValue {
() => {
// Module: crate::errors
// Provides: {"InvalidValue"}
// Dependencies: {}
# [doc = " Parse errors relating to invalid parameter values or salts."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum InvalidValue { # [doc = " Character is not in the allowed set."] InvalidChar (char) , # [doc = " Format is invalid."] InvalidFormat , # [doc = " Value is malformed."] Malformed , # [doc = " Value exceeds the maximum allowed length."] TooLong , # [doc = " Value does not satisfy the minimum length."] TooShort , }
};
}
