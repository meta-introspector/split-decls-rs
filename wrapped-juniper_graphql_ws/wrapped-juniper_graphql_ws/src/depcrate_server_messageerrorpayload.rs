// Generated macro for ErrorPayload (struct)
macro_rules! Depcrate_server_messageErrorPayload {
() => {
// Module: crate::server_message
// Provides: {"ErrorPayload"}
// Dependencies: {}
# [doc = " Payload for errors that can happen before execution."] # [doc = ""] # [doc = " Errors that happen during execution are instead sent to the client via"] # [doc = " [`graphql_ws::DataPayload`] or [`graphql_transport_ws::NextPayload`]. [`ErrorPayload`] is a"] # [doc = " wrapper for an owned [`GraphQLError`]."] # [doc = ""] # [doc = " [`graphql_transport_ws::NextPayload`]: crate::graphql_transport_ws::NextPayload"] # [doc = " [`graphql_ws::DataPayload`]: crate::graphql_ws::DataPayload"] # [derive (Debug)] # [debug ("{error:?}")] pub struct ErrorPayload { _execution_params : Option < Box < dyn Any + Send > > , error : GraphQLError , _pinned : PhantomPinned , }
};
}
