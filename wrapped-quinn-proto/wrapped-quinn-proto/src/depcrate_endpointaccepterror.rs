// Generated macro for AcceptError (struct)
macro_rules! Depcrate_endpointAcceptError {
() => {
// Module: crate::endpoint
// Provides: {"AcceptError"}
// Dependencies: {}
# [doc = " Error type for attempting to accept an [`Incoming`]"] # [derive (Debug)] pub struct AcceptError { # [doc = " Underlying error describing reason for failure"] pub cause : ConnectionError , # [doc = " Optional response to transmit back"] pub response : Option < Transmit > , }
};
}
