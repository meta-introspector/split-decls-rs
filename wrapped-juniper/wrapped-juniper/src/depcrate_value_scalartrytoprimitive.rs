// Generated macro for TryToPrimitive (trait)
macro_rules! Depcrate_value_scalarTryToPrimitive {
() => {
// Module: crate::value::scalar
// Provides: {"TryToPrimitive"}
// Dependencies: {}
# [doc = " Fallible representation of a [`ScalarValue`] as one of the types it consists of, or derived ones"] # [doc = " from them."] # [doc = ""] # [doc = " # Implementation"] # [doc = ""] # [doc = " Implementing this trait for a type allows to specify this type directly in the `from_input()`"] # [doc = " function when implementing a [`GraphQLScalar`] via [derive macro](macro@GraphQLScalar)."] # [doc = ""] # [doc = " `#[derive(`[`ScalarValue`](macro@crate::ScalarValue)`)]` automatically implements this trait for"] # [doc = " all the required primitive types if `#[to_<type>]` and `#[as_<type>]` attributes are specified."] pub trait TryToPrimitive < 'me , T : 'me > { # [doc = " Error if this [`ScalarValue`] doesn't represent the expected type."] type Error : 'me ; # [doc = " Tries to represent this [`ScalarValue`] as the expected type."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If this [`ScalarValue`] doesn't represent the expected type."] fn try_to_primitive (& 'me self) -> Result < T , Self :: Error > ; }
};
}
