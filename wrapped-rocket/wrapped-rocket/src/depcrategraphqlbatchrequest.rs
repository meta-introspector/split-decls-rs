// Generated macro for GraphQLBatchRequest (struct)
macro_rules! DepcrateGraphQLBatchRequest {
() => {
// Module: crate
// Provides: {"GraphQLBatchRequest"}
// Dependencies: {}
# [doc = " A batch request which can be extracted from a request's body."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[rocket::post(\"/graphql\", data = \"<request>\", format = \"application/json\", rank = 1)]"] # [doc = " async fn graphql_request(schema: State<'_, ExampleSchema>, request: BatchRequest) -> Response {"] # [doc = "     request.execute(&schema).await"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct GraphQLBatchRequest (pub async_graphql :: BatchRequest) ;
};
}
