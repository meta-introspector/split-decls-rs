// Generated macro for LmsDeserializeError (enum)
macro_rules! Depcrate_errorLmsDeserializeError {
() => {
// Module: crate::error
// Provides: {"LmsDeserializeError"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Eq , PartialEq)] # [doc = " The error returned by `TryFrom<&[u8]>` impls"] pub enum LmsDeserializeError { # [doc = " Length of the slice was `< 4` and no algorithm can be parsed"] NoAlgorithm , # [doc = " The parsed algorithm does not match the requested deserialization"] WrongAlgorithm , # [doc = " The slice did not contain enough data"] TooShort , # [doc = " The slice contained too much data"] TooLong , # [doc = " The parsed `q` value was too large"] InvalidQ , }
};
}
