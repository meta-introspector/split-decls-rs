// Generated macro for graphiql_source (function)
macro_rules! Depcrategraphiql_source {
() => {
// Module: crate
// Provides: {"graphiql_source"}
// Dependencies: {}
# [doc = " Generates a [`RawHtml`] page containing [GraphiQL]."] # [doc = ""] # [doc = " This does not handle routing, so you can mount it on any endpoint."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use rocket::{response::content::RawHtml, routes};"] # [doc = ""] # [doc = " #[rocket::get(\"/graphiql\")]"] # [doc = " fn graphiql() -> RawHtml<String> {"] # [doc = "     juniper_rocket::graphiql_source(\"/graphql\", \"/subscriptions\")"] # [doc = " }"] # [doc = ""] # [doc = " let rocket = rocket::build().mount(\"/\", routes![graphiql]);"] # [doc = " ```"] # [doc = ""] # [doc = " [GraphiQL]: https://github.com/graphql/graphiql"] pub fn graphiql_source < 'a > (graphql_endpoint_url : & str , subscriptions_endpoint_url : impl Into < Option < & 'a str > > ,) -> RawHtml < String > { RawHtml (http :: graphiql :: graphiql_source (graphql_endpoint_url , subscriptions_endpoint_url . into () ,)) }
};
}
