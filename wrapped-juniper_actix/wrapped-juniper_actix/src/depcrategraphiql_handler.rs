// Generated macro for graphiql_handler (function)
macro_rules! Depcrategraphiql_handler {
() => {
// Module: crate
// Provides: {"graphiql_handler"}
// Dependencies: {}
# [doc = " Create a handler that replies with an HTML page containing GraphiQL. This does not handle routing, so you can mount it on any endpoint"] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " ```"] # [doc = " # use juniper_actix::graphiql_handler;"] # [doc = " # use actix_web::{web, App};"] # [doc = ""] # [doc = " let app = App::new()"] # [doc = "          .route(\"/\", web::get().to(|| graphiql_handler(\"/graphql\", Some(\"/graphql/subscriptions\"))));"] # [doc = " ```"] pub async fn graphiql_handler (graphql_endpoint_url : & str , subscriptions_endpoint_url : Option < & 'static str > ,) -> Result < HttpResponse , Error > { let html = graphiql_source (graphql_endpoint_url , subscriptions_endpoint_url) ; Ok (HttpResponse :: Ok () . content_type ("text/html; charset=utf-8") . body (html)) }
};
}
