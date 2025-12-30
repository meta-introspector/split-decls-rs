// Generated macro for playground (function)
macro_rules! Depcrateplayground {
() => {
// Module: crate
// Provides: {"playground"}
// Dependencies: {}
# [doc = " Creates a [`Handler`] that replies with an HTML page containing [GraphQL Playground]."] # [doc = ""] # [doc = " This does not handle routing, so you can mount it on any endpoint."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use axum::{routing::get, Router};"] # [doc = " use juniper_axum::playground;"] # [doc = ""] # [doc = " let app: Router = Router::new()"] # [doc = "     .route(\"/\", get(playground(\"/graphql\", \"/subscriptions\")));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Handler`]: axum::handler::Handler"] # [doc = " [GraphQL Playground]: https://github.com/prisma/graphql-playground"] pub fn playground < 'a , T : Into < Option < & 'a str > > > (graphql_endpoint_url : & str , subscriptions_endpoint_url : T ,) -> impl FnOnce () -> future :: Ready < Html < String > > + Clone + Send + use < T > { let html = Html (juniper :: http :: playground :: playground_source (graphql_endpoint_url , subscriptions_endpoint_url . into () ,)) ; | | future :: ready (html) }
};
}
