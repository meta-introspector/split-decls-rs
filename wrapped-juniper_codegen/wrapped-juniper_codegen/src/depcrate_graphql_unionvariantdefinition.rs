// Generated macro for VariantDefinition (struct)
macro_rules! Depcrate_graphql_unionVariantDefinition {
() => {
// Module: crate::graphql_union
// Provides: {"VariantDefinition"}
// Dependencies: {}
# [doc = " Definition of [GraphQL union][1] variant for code generation."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] struct VariantDefinition { # [doc = " Rust type that this [GraphQL union][1] variant resolves into."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] ty : syn :: Type , # [doc = " Rust code for value resolution of this [GraphQL union][1] variant."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] resolver_code : syn :: Expr , # [doc = " Rust code for checking whether [GraphQL union][1] should be resolved"] # [doc = " into this variant."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] resolver_check : syn :: Expr , # [doc = " Rust type of [`Context`] that this [GraphQL union][1] variant requires"] # [doc = " for resolution."] # [doc = ""] # [doc = " It's available only when code generation happens for Rust traits and a"] # [doc = " trait method contains context argument."] # [doc = ""] # [doc = " [`Context`]: juniper::Context"] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] context : Option < syn :: Type > , }
};
}
