// Generated macro for GraphQL (struct)
macro_rules! Depcrate_queryGraphQL {
() => {
// Module: crate::query
// Provides: {"GraphQL"}
// Dependencies: {}
# [doc = " A GraphQL query endpoint."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};"] # [doc = " use async_graphql_poem::GraphQL;"] # [doc = " use poem::{Route, post};"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn value(&self) -> i32 {"] # [doc = "         100"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;"] # [doc = ""] # [doc = " let schema = Schema::new(Query, EmptyMutation, EmptySubscription);"] # [doc = " let app = Route::new().at(\"/\", post(GraphQL::new(schema)));"] # [doc = " ```"] pub struct GraphQL < E > { executor : E , }
};
}
