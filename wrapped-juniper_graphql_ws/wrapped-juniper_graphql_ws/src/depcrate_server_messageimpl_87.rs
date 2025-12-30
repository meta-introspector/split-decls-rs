// Generated macro for impl_87 (impl)
macro_rules! Depcrate_server_messageimpl_87 {
() => {
// Module: crate::server_message
// Provides: {"impl_87"}
// Dependencies: {}
impl ErrorPayload { # [doc = " Creates a new [`ErrorPayload`] out of the provide `execution_params` and [`GraphQLError`]."] pub (crate) fn new (execution_params : Box < dyn Any + Send > , error : GraphQLError) -> Self { Self { _execution_params : Some (execution_params) , error , _pinned : PhantomPinned , } } # [doc = " Returns the contained [`GraphQLError`]."] pub fn graphql_error (& self) -> & GraphQLError { & self . error } }
};
}
