// Generated macro for playground_response (function)
macro_rules! Depcrateplayground_response {
() => {
// Module: crate
// Provides: {"playground_response"}
// Dependencies: {}
fn playground_response (graphql_endpoint_url : & 'static str , subscriptions_endpoint_url : Option < & 'static str > ,) -> http :: Response < Vec < u8 > > { http :: Response :: builder () . header ("content-type" , "text/html;charset=utf-8") . body (juniper :: http :: playground :: playground_source (graphql_endpoint_url , subscriptions_endpoint_url ,) . into_bytes () ,) . expect ("response is valid") }
};
}
