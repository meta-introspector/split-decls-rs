// Generated macro for IntoInputValue (trait)
macro_rules! Depcrate_astIntoInputValue {
() => {
// Module: crate::ast
// Provides: {"IntoInputValue"}
// Dependencies: {}
# [doc = " Conversion into an [`InputValue`]."] # [doc = ""] # [doc = " This trait exists to work around [orphan rules] and allow to specify custom efficient"] # [doc = " conversions whenever some custom [`ScalarValue`] is involved"] # [doc = " (`impl IntoInputValue<CustomScalarValue> for ForeignType` would work, while"] # [doc = " `impl From<ForeignType> for InputValue<CustomScalarValue>` wound not)."] # [doc = ""] # [doc = " This trait is used inside [`graphql_input_value!`] macro expansion and implementing it allows to"] # [doc = " put values of the implementor type there."] # [doc = ""] # [doc = " [`graphql_input_value!`]: crate::graphql_input_value"] # [doc = " [orphan rules]: https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules"] pub trait IntoInputValue < S > { # [doc = " Converts this value into an [`InputValue`]."] # [must_use] fn into_input_value (self) -> InputValue < S > ; }
};
}
