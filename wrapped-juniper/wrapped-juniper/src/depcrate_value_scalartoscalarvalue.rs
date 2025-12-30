// Generated macro for ToScalarValue (trait)
macro_rules! Depcrate_value_scalarToScalarValue {
() => {
// Module: crate::value::scalar
// Provides: {"ToScalarValue"}
// Dependencies: {}
# [doc = " Conversion of a Rust data type into a [`ScalarValue`]."] # [doc = ""] # [doc = " # Implementation"] # [doc = ""] # [doc = " Implementing this trait for a type allows to specify this type directly in the `to_output()`"] # [doc = " function when implementing a [`GraphQLScalar`] via [derive macro](macro@GraphQLScalar)."] # [doc = ""] # [doc = " Also, `#[derive(`[`GraphQLScalar`](macro@GraphQLScalar)`)]` automatically implements this trait"] # [doc = " for a type."] pub trait ToScalarValue < S = DefaultScalarValue > { # [doc = " Converts this value into a [`ScalarValue`]."] # [must_use] fn to_scalar_value (& self) -> S ; }
};
}
