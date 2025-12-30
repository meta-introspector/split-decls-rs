// Generated macro for ExecutionParams (struct)
macro_rules! Depcrate_graphql_transport_wsExecutionParams {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"ExecutionParams"}
// Dependencies: {}
struct ExecutionParams < S : Schema > { subscribe_payload : SubscribePayload < S :: ScalarValue > , config : Arc < ConnectionConfig < S :: Context > > , schema : S , }
};
}
