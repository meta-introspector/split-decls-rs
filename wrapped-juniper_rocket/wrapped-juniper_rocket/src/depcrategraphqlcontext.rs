// Generated macro for GraphQLContext (struct)
macro_rules! DepcrateGraphQLContext {
() => {
// Module: crate
// Provides: {"GraphQLContext"}
// Dependencies: {}
# [doc = " [`FromForm::Context`] of a [`GraphQLRequest`]."] pub struct GraphQLContext < 'f , S : ScalarValue > { opts : Options , query : Option < String > , operation_name : Option < String > , variables : Option < InputValue < S > > , errors : Errors < 'f > , }
};
}
