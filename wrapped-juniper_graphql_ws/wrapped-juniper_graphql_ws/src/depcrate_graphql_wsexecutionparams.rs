// Generated macro for ExecutionParams (struct)
macro_rules! Depcrate_graphql_wsExecutionParams {
() => {
// Module: crate::graphql_ws
// Provides: {"ExecutionParams"}
// Dependencies: {}
struct ExecutionParams < S : Schema > { start_payload : StartPayload < S :: ScalarValue > , config : Arc < ConnectionConfig < S :: Context > > , schema : S , }
};
}
