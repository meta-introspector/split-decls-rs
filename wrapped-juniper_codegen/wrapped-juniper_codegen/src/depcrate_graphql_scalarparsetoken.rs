// Generated macro for ParseToken (enum)
macro_rules! Depcrate_graphql_scalarParseToken {
() => {
// Module: crate::graphql_scalar
// Provides: {"ParseToken"}
// Dependencies: {}
# [doc = " Representation of [`ParseScalarValue::from_str`] method."] # [doc = ""] # [doc = " [`ParseScalarValue::from_str`]: juniper::ParseScalarValue::from_str"] # [derive (Clone , Debug)] enum ParseToken { # [doc = " Custom method."] Custom (syn :: ExprPath) , # [doc = " Tries to parse using [`syn::Type`]s [`ParseScalarValue`] impls until"] # [doc = " first success."] # [doc = ""] # [doc = " [`ParseScalarValue`]: juniper::ParseScalarValue"] Delegated (Vec < syn :: Type >) , }
};
}
