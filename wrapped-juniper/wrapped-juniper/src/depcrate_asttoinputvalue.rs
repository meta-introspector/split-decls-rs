// Generated macro for ToInputValue (trait)
macro_rules! Depcrate_astToInputValue {
() => {
// Module: crate::ast
// Provides: {"ToInputValue"}
// Dependencies: {}
# [doc = " Losslessly clones a Rust data type into an [`InputValue`]."] pub trait ToInputValue < S = DefaultScalarValue > { # [doc = " Performs the conversion."] fn to_input_value (& self) -> InputValue < S > ; }
};
}
