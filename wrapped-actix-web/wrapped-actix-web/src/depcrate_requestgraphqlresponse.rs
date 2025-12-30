// Generated macro for GraphQLResponse (struct)
macro_rules! Depcrate_requestGraphQLResponse {
() => {
// Module: crate::request
// Provides: {"GraphQLResponse"}
// Dependencies: {}
# [doc = " Responder for a GraphQL response."] # [doc = ""] # [doc = " This contains a batch response, but since regular responses are a type of"] # [doc = " batch response it works for both."] pub struct GraphQLResponse (pub async_graphql :: BatchResponse) ;
};
}
