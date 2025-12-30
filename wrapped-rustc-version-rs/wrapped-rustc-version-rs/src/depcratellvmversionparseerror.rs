// Generated macro for LlvmVersionParseError (enum)
macro_rules! DepcrateLlvmVersionParseError {
() => {
// Module: crate
// Provides: {"LlvmVersionParseError"}
// Dependencies: {}
# [doc = " LLVM Version Parse Error"] # [derive (Debug)] pub enum LlvmVersionParseError { # [doc = " An error occurred in parsing a version component as an integer"] ParseIntError (num :: ParseIntError) , # [doc = " A version component must not have leading zeros"] ComponentMustNotHaveLeadingZeros , # [doc = " A version component has a sign"] ComponentMustNotHaveSign , # [doc = " Minor version component must be zero on LLVM versions later than 4.0"] MinorVersionMustBeZeroAfter4 , # [doc = " Minor version component is required on LLVM versions earlier than 4.0"] MinorVersionRequiredBefore4 , # [doc = " Too many components"] TooManyComponents , }
};
}
