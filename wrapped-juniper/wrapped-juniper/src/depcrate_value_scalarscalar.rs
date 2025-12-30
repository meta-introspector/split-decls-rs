// Generated macro for Scalar (struct)
macro_rules! Depcrate_value_scalarScalar {
() => {
// Module: crate::value::scalar
// Provides: {"Scalar"}
// Dependencies: {}
# [doc = " Transparent wrapper over a value, indicating it being a [`ScalarValue`]."] # [doc = ""] # [doc = " Used in [`GraphQLScalar`] definitions to distinguish a concrete type for a generic"] # [doc = " [`ScalarValue`], since Rust type inference fail do so for a generic value directly in macro"] # [doc = " expansions."] # [derive (Debug , Deref , RefCast)] # [repr (transparent)] pub struct Scalar < T : ScalarValue > (T) ;
};
}
