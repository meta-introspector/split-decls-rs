// Generated macro for playground_source (function)
macro_rules! Depcrateplayground_source {
() => {
// Module: crate
// Provides: {"playground_source"}
// Dependencies: {}
# [doc = " Generates a [`RawHtml`] page containing [GraphQL Playground]."] # [doc = ""] # [doc = " This does not handle routing, so you can mount it on any endpoint."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use rocket::{response::content::RawHtml, routes};"] # [doc = ""] # [doc = " #[rocket::get(\"/playground\")]"] # [doc = " fn playground() -> RawHtml<String> {"] # [doc = "     juniper_rocket::playground_source(\"/graphql\", \"/subscriptions\")"] # [doc = " }"] # [doc = ""] # [doc = " let rocket = rocket::build().mount(\"/\", routes![playground]);"] # [doc = " ```"] # [doc = ""] # [doc = " [GraphQL Playground]: https://github.com/prisma/graphql-playground"] pub fn playground_source < 'a > (graphql_endpoint_url : & str , subscriptions_endpoint_url : impl Into < Option < & 'a str > > ,) -> RawHtml < String > { RawHtml (http :: playground :: playground_source (graphql_endpoint_url , subscriptions_endpoint_url . into () ,)) }
};
}
