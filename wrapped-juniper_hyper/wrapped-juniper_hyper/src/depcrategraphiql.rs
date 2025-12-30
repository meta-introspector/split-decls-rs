// Generated macro for graphiql (function)
macro_rules! Depcrategraphiql {
() => {
// Module: crate
// Provides: {"graphiql"}
// Dependencies: {}
# [doc = " Generates a [`Response`] page containing [GraphiQL]."] # [doc = ""] # [doc = " This does not handle routing, so you can mount it on any endpoint."] # [doc = ""] # [doc = " [GraphiQL]: https://github.com/graphql/graphiql"] pub async fn graphiql (graphql_endpoint : & str , subscriptions_endpoint : Option < & str > ,) -> Response < String > { let mut resp = new_html_response (StatusCode :: OK) ; * resp . body_mut () = juniper :: http :: graphiql :: graphiql_source (graphql_endpoint , subscriptions_endpoint) ; resp }
};
}
