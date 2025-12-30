// Generated macro for graphiql (function)
macro_rules! Depcrategraphiql {
() => {
// Module: crate
// Provides: {"graphiql"}
// Dependencies: {}
# [doc = " Creates a [`Handler`] that replies with an HTML page containing [GraphiQL]."] # [doc = ""] # [doc = " This does not handle routing, so you can mount it on any endpoint."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use axum::{routing::get, Router};"] # [doc = " use juniper_axum::graphiql;"] # [doc = ""] # [doc = " let app: Router = Router::new()"] # [doc = "     .route(\"/\", get(graphiql(\"/graphql\", \"/subscriptions\")));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Handler`]: axum::handler::Handler"] # [doc = " [GraphiQL]: https://github.com/graphql/graphiql"] pub fn graphiql < 'a , T : Into < Option < & 'a str > > > (graphql_endpoint_url : & str , subscriptions_endpoint_url : T ,) -> impl FnOnce () -> future :: Ready < Html < String > > + Clone + Send + use < T > { let html = Html (juniper :: http :: graphiql :: graphiql_source (graphql_endpoint_url , subscriptions_endpoint_url . into () ,)) ; | | future :: ready (html) }
};
}
