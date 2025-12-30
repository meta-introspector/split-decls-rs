// Generated macro for graphiql_filter (function)
macro_rules! Depcrategraphiql_filter {
() => {
// Module: crate
// Provides: {"graphiql_filter"}
// Dependencies: {}
# [doc = " Create a filter that replies with an HTML page containing GraphiQL. This does not handle routing, so you can mount it on any endpoint."] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " ```"] # [doc = " # use warp::Filter;"] # [doc = " # use juniper_warp::graphiql_filter;"] # [doc = " #"] # [doc = " let graphiql_route = warp::path(\"graphiql\").and(graphiql_filter(\"/graphql\","] # [doc = " None));"] # [doc = " ```"] # [doc = ""] # [doc = " Or with subscriptions support, provide the subscriptions endpoint URL:"] # [doc = ""] # [doc = " ```"] # [doc = " # use warp::Filter;"] # [doc = " # use juniper_warp::graphiql_filter;"] # [doc = " #"] # [doc = " let graphiql_route = warp::path(\"graphiql\").and(graphiql_filter(\"/graphql\","] # [doc = " Some(\"ws://localhost:8080/subscriptions\")));"] # [doc = " ```"] pub fn graphiql_filter (graphql_endpoint_url : & 'static str , subscriptions_endpoint : Option < & 'static str > ,) -> warp :: filters :: BoxedFilter < (http :: Response < Vec < u8 > > ,) > { warp :: any () . map (move | | graphiql_response (graphql_endpoint_url , subscriptions_endpoint)) . boxed () }
};
}
