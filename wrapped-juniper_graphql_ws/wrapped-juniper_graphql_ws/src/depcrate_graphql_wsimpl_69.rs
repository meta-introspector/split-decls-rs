// Generated macro for impl_69 (impl)
macro_rules! Depcrate_graphql_wsimpl_69 {
() => {
// Module: crate::graphql_ws
// Provides: {"impl_69"}
// Dependencies: {}
impl < S , I > Connection < S , I > where S : Schema , I : Init < S :: ScalarValue , S :: Context > , { # [doc = " Creates a new connection, which is a sink for `TryInto<ClientMessage>` and a stream of `ServerMessage`."] # [doc = ""] # [doc = " The `schema` argument should typically be an `Arc<RootNode<...>>`."] # [doc = ""] # [doc = " The `init` argument is used to provide the context and additional configuration for"] # [doc = " connections. This can be a `ConnectionConfig` if the context and configuration are already"] # [doc = " known, or it can be a closure that gets executed asynchronously when the client sends the"] # [doc = " ConnectionInit message. Using a closure allows you to perform authentication based on the"] # [doc = " parameters provided by the client."] pub fn new (schema : S , init : I) -> Self { Self { reactions : SelectAll :: new () , stream_waker : None , sink_state : ConnectionSinkState :: Ready { state : ConnectionState :: PreInit { init , schema } , } , } } }
};
}
