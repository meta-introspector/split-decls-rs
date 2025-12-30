// Generated macro for IntoValue (trait)
macro_rules! Depcrate_valueIntoValue {
() => {
// Module: crate::value
// Provides: {"IntoValue"}
// Dependencies: {}
# [doc = " Conversion into a [`Value`]."] # [doc = ""] # [doc = " This trait exists to work around [orphan rules] and allow to specify custom efficient"] # [doc = " conversions whenever some custom [`ScalarValue`] is involved"] # [doc = " (`impl IntoValue<CustomScalarValue> for ForeignType` would work, while"] # [doc = " `impl From<ForeignType> for Value<CustomScalarValue>` wound not)."] # [doc = ""] # [doc = " This trait is used inside [`graphql_value!`] macro expansion and implementing it allows to"] # [doc = " put values of the implementor type there."] # [doc = ""] # [doc = " [`graphql_value!`]: crate::graphql_value"] # [doc = " [orphan rules]: https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules"] pub trait IntoValue < S > { # [doc = " Converts this value into a [`Value`]."] # [must_use] fn into_value (self) -> Value < S > ; }
};
}
