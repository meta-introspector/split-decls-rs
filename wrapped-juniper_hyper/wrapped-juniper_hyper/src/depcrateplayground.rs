// Generated macro for playground (function)
macro_rules! Depcrateplayground {
() => {
// Module: crate
// Provides: {"playground"}
// Dependencies: {}
# [doc = " Generates a [`Response`] page containing [GraphQL Playground]."] # [doc = ""] # [doc = " This does not handle routing, so you can mount it on any endpoint."] # [doc = ""] # [doc = " [GraphQL Playground]: https://github.com/prisma/graphql-playground"] pub async fn playground (graphql_endpoint : & str , subscriptions_endpoint : Option < & str > ,) -> Response < String > { let mut resp = new_html_response (StatusCode :: OK) ; * resp . body_mut () = juniper :: http :: playground :: playground_source (graphql_endpoint , subscriptions_endpoint) ; resp }
};
}
