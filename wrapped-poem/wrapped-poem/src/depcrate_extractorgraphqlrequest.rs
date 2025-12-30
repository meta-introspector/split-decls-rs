// Generated macro for GraphQLRequest (struct)
macro_rules! Depcrate_extractorGraphQLRequest {
() => {
// Module: crate::extractor
// Provides: {"GraphQLRequest"}
// Dependencies: {}
# [doc = " An extractor for GraphQL request."] # [doc = ""] # [doc = " You can just use the extractor as in the example below, but I would"] # [doc = " recommend using the [`GraphQL`](crate::GraphQL) endpoint because it is"] # [doc = " easier to integrate."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};"] # [doc = " use async_graphql_poem::GraphQLRequest;"] # [doc = " use poem::{"] # [doc = "     EndpointExt, Route, handler,"] # [doc = "     middleware::AddData,"] # [doc = "     post,"] # [doc = "     web::{Data, Json},"] # [doc = " };"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn value(&self) -> i32 {"] # [doc = "         100"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;"] # [doc = ""] # [doc = " #[handler]"] # [doc = " async fn index(req: GraphQLRequest, schema: Data<&MySchema>) -> Json<async_graphql::Response> {"] # [doc = "     Json(schema.execute(req.0).await)"] # [doc = " }"] # [doc = ""] # [doc = " let schema = Schema::new(Query, EmptyMutation, EmptySubscription);"] # [doc = " let app = Route::new().at(\"/\", post(index.with(AddData::new(schema))));"] # [doc = " ```"] pub struct GraphQLRequest (pub async_graphql :: Request) ;
};
}
