// Generated macro for graphiql_response (function)
macro_rules! Depcrategraphiql_response {
() => {
// Module: crate
// Provides: {"graphiql_response"}
// Dependencies: {}
fn graphiql_response (graphql_endpoint_url : & 'static str , subscriptions_endpoint : Option < & 'static str > ,) -> http :: Response < Vec < u8 > > { http :: Response :: builder () . header ("content-type" , "text/html;charset=utf-8") . body (juniper :: http :: graphiql :: graphiql_source (graphql_endpoint_url , subscriptions_endpoint) . into_bytes () ,) . expect ("response is valid") }
};
}
