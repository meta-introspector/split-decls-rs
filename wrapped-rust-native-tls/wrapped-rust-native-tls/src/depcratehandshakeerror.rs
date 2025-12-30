// Generated macro for HandshakeError (enum)
macro_rules! DepcrateHandshakeError {
() => {
// Module: crate
// Provides: {"HandshakeError"}
// Dependencies: {}
# [doc = " An error returned from `ClientBuilder::handshake`."] # [derive (Debug)] pub enum HandshakeError < S > { # [doc = " A fatal error."] Failure (Error) , # [doc = " A stream interrupted midway through the handshake process due to a"] # [doc = " `WouldBlock` error."] # [doc = ""] # [doc = " Note that this is not a fatal error and it should be safe to call"] # [doc = " `handshake` at a later time once the stream is ready to perform I/O"] # [doc = " again."] WouldBlock (MidHandshakeTlsStream < S >) , }
};
}
