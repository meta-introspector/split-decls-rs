// Generated macro for playground_handler (function)
macro_rules! Depcrateplayground_handler {
() => {
// Module: crate
// Provides: {"playground_handler"}
// Dependencies: {}
# [doc = " Create a handler that replies with an HTML page containing GraphQL Playground. This does not handle routing, so you cant mount it on any endpoint."] pub async fn playground_handler (graphql_endpoint_url : & str , subscriptions_endpoint_url : Option < & 'static str > ,) -> Result < HttpResponse , Error > { let html = playground_source (graphql_endpoint_url , subscriptions_endpoint_url) ; Ok (HttpResponse :: Ok () . content_type ("text/html; charset=utf-8") . body (html)) }
};
}
