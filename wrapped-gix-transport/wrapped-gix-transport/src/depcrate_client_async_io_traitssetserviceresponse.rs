// Generated macro for SetServiceResponse (struct)
macro_rules! Depcrate_client_async_io_traitsSetServiceResponse {
() => {
// Module: crate::client::async_io::traits
// Provides: {"SetServiceResponse"}
// Dependencies: {}
# [doc = " The response of the [`handshake()`][Transport::handshake()] method."] pub struct SetServiceResponse < 'a > { # [doc = " The protocol the service can provide. May be different from the requested one"] pub actual_protocol : Protocol , # [doc = " The capabilities parsed from the server response."] pub capabilities : Capabilities , # [doc = " In protocol version one, this is set to a list of refs and their peeled counterparts."] pub refs : Option < Box < dyn ReadlineBufRead + Unpin + 'a > > , }
};
}
