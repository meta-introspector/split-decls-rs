// Generated macro for GraphQLSubscription (struct)
macro_rules! Depcrate_subscriptionGraphQLSubscription {
() => {
// Module: crate::subscription
// Provides: {"GraphQLSubscription"}
// Dependencies: {}
# [doc = " A GraphQL subscription endpoint."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use async_graphql::{EmptyMutation, Object, Schema, Subscription};"] # [doc = " use async_graphql_poem::GraphQLSubscription;"] # [doc = " use futures_util::{Stream, stream};"] # [doc = " use poem::{Route, get};"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn value(&self) -> i32 {"] # [doc = "         100"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " struct Subscription;"] # [doc = ""] # [doc = " #[Subscription]"] # [doc = " impl Subscription {"] # [doc = "     async fn values(&self) -> impl Stream<Item = i32> {"] # [doc = "         stream::iter(vec![1, 2, 3, 4, 5])"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " type MySchema = Schema<Query, EmptyMutation, Subscription>;"] # [doc = ""] # [doc = " let schema = Schema::new(Query, EmptyMutation, Subscription);"] # [doc = " let app = Route::new().at(\"/ws\", get(GraphQLSubscription::new(schema)));"] # [doc = " ```"] pub struct GraphQLSubscription < E > { executor : E , }
};
}
