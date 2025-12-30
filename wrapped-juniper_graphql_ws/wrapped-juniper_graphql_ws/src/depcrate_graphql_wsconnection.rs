// Generated macro for Connection (struct)
macro_rules! Depcrate_graphql_wsConnection {
() => {
// Module: crate::graphql_ws
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " Implements the graphql-ws protocol. This is a sink for `TryInto<ClientMessage>` and a stream of"] # [doc = " `ServerMessage`."] pub struct Connection < S : Schema , I : Init < S :: ScalarValue , S :: Context > > { reactions : SelectAll < BoxStream < 'static , Reaction < S > > > , stream_waker : Option < Waker > , sink_state : ConnectionSinkState < S , I > , }
};
}
