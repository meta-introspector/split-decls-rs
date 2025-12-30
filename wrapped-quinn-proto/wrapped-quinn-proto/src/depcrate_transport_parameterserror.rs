// Generated macro for Error (enum)
macro_rules! Depcrate_transport_parametersError {
() => {
// Module: crate::transport_parameters
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors encountered while decoding `TransportParameters`"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Error)] pub enum Error { # [doc = " Parameters that are semantically invalid"] # [error ("parameter had illegal value")] IllegalValue , # [doc = " Catch-all error for problems while decoding transport parameters"] # [error ("parameters were malformed")] Malformed , }
};
}
