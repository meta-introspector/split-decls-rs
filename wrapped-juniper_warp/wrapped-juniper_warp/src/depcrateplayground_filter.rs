// Generated macro for playground_filter (function)
macro_rules! Depcrateplayground_filter {
() => {
// Module: crate
// Provides: {"playground_filter"}
// Dependencies: {}
# [doc = " Create a filter that replies with an HTML page containing GraphQL Playground. This does not handle routing, so you can mount it on any endpoint."] pub fn playground_filter (graphql_endpoint_url : & 'static str , subscriptions_endpoint_url : Option < & 'static str > ,) -> warp :: filters :: BoxedFilter < (http :: Response < Vec < u8 > > ,) > { warp :: any () . map (move | | playground_response (graphql_endpoint_url , subscriptions_endpoint_url)) . boxed () }
};
}
