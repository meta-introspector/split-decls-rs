// Generated macro for Methods (enum)
macro_rules! Depcrate_graphql_scalarMethods {
() => {
// Module: crate::graphql_scalar
// Provides: {"Methods"}
// Dependencies: {}
# [doc = " Methods representing [GraphQL scalar][1]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Scalars"] enum Methods { # [doc = " [GraphQL scalar][1] represented with only custom resolvers."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Scalars"] Custom { # [doc = " Function provided with `#[graphql(to_output_with = ...)]`."] to_output : syn :: ExprPath , # [doc = " Function provided with `#[graphql(from_input_with = ...)]`."] from_input : syn :: ExprPath , # [doc = " [`ParseToken`] provided with `#[graphql(parse_token_with = ...)]`"] # [doc = " or `#[graphql(parse_token(...))]`."] parse_token : ParseToken , } , # [doc = " [GraphQL scalar][1] maybe partially represented with custom resolver."] # [doc = " Other methods are used from [`Field`]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Scalars"] Delegated { # [doc = " Function provided with `#[graphql(to_output_with = ...)]`."] to_output : Option < syn :: ExprPath > , # [doc = " Function provided with `#[graphql(from_input_with = ...)]`."] from_input : Option < syn :: ExprPath > , # [doc = " [`ParseToken`] provided with `#[graphql(parse_token_with = ...)]`"] # [doc = " or `#[graphql(parse_token(...))]`."] parse_token : Option < ParseToken > , # [doc = " [`Field`] to resolve not provided methods."] field : Box < Field > , } , }
};
}
