// Generated macro for FromScalarValue (trait)
macro_rules! Depcrate_value_scalarFromScalarValue {
() => {
// Module: crate::value::scalar
// Provides: {"FromScalarValue"}
// Dependencies: {}
# [doc = " Parsing of a [`ScalarValue`] into a Rust data type."] # [doc = ""] # [doc = " The conversion _can_ fail, and must in that case return an [`Err`]."] # [doc = ""] # [doc = " Use the [`ScalarValue::try_to()`] method as a shortcut for this conversion."] # [doc = ""] # [doc = " # Implementation"] # [doc = ""] # [doc = " Implementing this trait for a type allows to specify this type directly in the `from_input()`"] # [doc = " function when implementing a [`GraphQLScalar`] via [derive macro](macro@GraphQLScalar)."] # [doc = ""] # [doc = " Also, `#[derive(`[`GraphQLScalar`](macro@GraphQLScalar)`)]` automatically implements this trait"] # [doc = " for a type."] pub trait FromScalarValue < 's , S : 's = DefaultScalarValue > : Sized { # [doc = " Parsing error of a [`ScalarValue`]."] type Error : IntoFieldError < S > + 's ; # [doc = " Parses the provided [`ScalarValue`]."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If this type cannot be parsed from the provided [`ScalarValue`]."] fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > ; }
};
}
