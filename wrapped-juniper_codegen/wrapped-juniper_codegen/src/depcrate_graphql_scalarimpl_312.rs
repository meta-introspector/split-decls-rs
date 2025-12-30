// Generated macro for impl_312 (impl)
macro_rules! Depcrate_graphql_scalarimpl_312 {
() => {
// Module: crate::graphql_scalar
// Provides: {"impl_312"}
// Dependencies: {}
impl Field { # [doc = " [`syn::Type`] of this [`Field`]."] fn ty (& self) -> & syn :: Type { match self { Self :: Named (f) | Self :: Unnamed (f) => & f . ty , } } # [doc = " Closure to construct [GraphQL scalar][1] struct from [`Field`]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Scalars"] fn closure_constructor (& self) -> TokenStream { match self { Field :: Named (syn :: Field { ident , .. }) => { quote ! { | v | Self { # ident : v } } } Field :: Unnamed (_) => quote ! { Self } , } } }
};
}
