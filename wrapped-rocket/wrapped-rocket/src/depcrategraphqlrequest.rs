// Generated macro for GraphQLRequest (struct)
macro_rules! DepcrateGraphQLRequest {
() => {
// Module: crate
// Provides: {"GraphQLRequest"}
// Dependencies: {}
# [doc = " A GraphQL request which can be extracted from the request's body."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[rocket::post(\"/graphql\", data = \"<request>\", format = \"application/json\", rank = 2)]"] # [doc = " async fn graphql_request(schema: State<'_, ExampleSchema>, request: Request) -> Result<Response, Status> {"] # [doc = "     request.execute(&schema).await"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct GraphQLRequest (pub async_graphql :: Request) ;
};
}
