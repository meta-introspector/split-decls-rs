// Generated macro for Value (enum)
macro_rules! Depcrate_common_defaultValue {
() => {
// Module: crate::common::default
// Provides: {"Value"}
// Dependencies: {}
# [doc = " Representation of a [GraphQL default value][0] for code generation."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#DefaultValue"] # [derive (Clone , Debug , Default)] pub (crate) enum Value { # [doc = " [`Default`] implementation should be used."] # [default] Default , # [doc = " Explicit [`Expr`]ession to be used as the [default value][0]."] # [doc = ""] # [doc = " [`Expr`]: syn::Expr"] # [doc = " [0]: https://spec.graphql.org/October2021#DefaultValue"] Expr (Box < syn :: Expr >) , }
};
}
