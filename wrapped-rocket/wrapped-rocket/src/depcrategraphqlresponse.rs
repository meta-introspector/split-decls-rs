// Generated macro for GraphQLResponse (struct)
macro_rules! DepcrateGraphQLResponse {
() => {
// Module: crate
// Provides: {"GraphQLResponse"}
// Dependencies: {}
# [doc = " Wrapper around `async-graphql::Response` that is a Rocket responder so it"] # [doc = " can be returned from a routing function in Rocket."] # [doc = ""] # [doc = " It contains a `BatchResponse` but since a response is a type of batch"] # [doc = " response it works for both."] # [derive (Debug)] pub struct GraphQLResponse (pub async_graphql :: BatchResponse) ;
};
}
