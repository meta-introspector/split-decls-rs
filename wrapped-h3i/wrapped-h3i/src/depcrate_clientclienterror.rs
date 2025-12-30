// Generated macro for ClientError (enum)
macro_rules! Depcrate_clientClientError {
() => {
// Module: crate::client
// Provides: {"ClientError"}
// Dependencies: {}
# [derive (Debug , Serialize)] # [doc = " Represents different errors that can occur when the h3i client runs."] pub enum ClientError { # [doc = " An error during the QUIC handshake."] HandshakeFail , # [doc = " An error during HTTP/3 exchanges."] HttpFail , # [doc = " Some other type of error."] Other (String) , }
};
}
