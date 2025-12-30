// Generated macro for TypeOrIdent (enum)
macro_rules! Depcrate_graphql_scalarTypeOrIdent {
() => {
// Module: crate::graphql_scalar
// Provides: {"TypeOrIdent"}
// Dependencies: {}
# [doc = " [`syn::Type`] in case of `#[graphql_scalar]` or [`syn::Ident`] in case of"] # [doc = " `#[derive(GraphQLScalar)]`."] # [derive (Clone)] enum TypeOrIdent { # [doc = " [`syn::Type`]."] Type (Box < syn :: Type >) , # [doc = " [`syn::Ident`]."] Ident (syn :: Ident) , }
};
}
