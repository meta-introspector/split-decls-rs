// Generated macro for VariantAttr (struct)
macro_rules! Depcrate_graphql_unionVariantAttr {
() => {
// Module: crate::graphql_union
// Provides: {"VariantAttr"}
// Dependencies: {}
# [doc = " Available arguments behind `#[graphql]` attribute when generating code for"] # [doc = " [GraphQL union][1]'s variant."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] # [derive (Debug , Default)] struct VariantAttr { # [doc = " Explicitly specified marker for the variant/field being ignored and not"] # [doc = " included into [GraphQL union][1]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] ignore : Option < SpanContainer < syn :: Ident > > , # [doc = " Explicitly specified external resolver function for this [GraphQL union][1] variant."] # [doc = ""] # [doc = " If absent, then macro will generate the code which just returns the variant inner value."] # [doc = " Usually, specifying an external resolver function has sense, when some custom resolving"] # [doc = " logic is involved."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Unions"] external_resolver : Option < SpanContainer < syn :: ExprPath > > , }
};
}
