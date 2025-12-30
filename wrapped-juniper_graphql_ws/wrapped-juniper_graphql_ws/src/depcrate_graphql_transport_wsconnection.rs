// Generated macro for Connection (struct)
macro_rules! Depcrate_graphql_transport_wsConnection {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " Implements the `graphql-ws` protocol."] # [doc = " This is a sink for `TryInto<Input>` messages and a stream of `Output` messages."] pub struct Connection < S : Schema , I : Init < S :: ScalarValue , S :: Context > > { reactions : SelectAll < BoxStream < 'static , Output < S :: ScalarValue > > > , stream_waker : Option < Waker > , stream_terminated : bool , sink_state : ConnectionSinkState < S , I > , }
};
}
