// Generated macro for rustls_handshake_kind (enum)
macro_rules! Depcrate_enumsrustls_handshake_kind {
() => {
// Module: crate::enums
// Provides: {"rustls_handshake_kind"}
// Dependencies: {}
# [derive (Debug , Default)] # [repr (C)] # [doc = " Describes which sort of handshake happened."] pub enum rustls_handshake_kind { # [doc = " The type of handshake could not be determined."] # [doc = ""] # [doc = " This variant should not be used."] # [default] Unknown = 0x0 , # [doc = " A full TLS handshake."] # [doc = ""] # [doc = " This is the typical TLS connection initiation process when resumption is"] # [doc = " not yet unavailable, and the initial client hello was accepted by the server."] Full = 0x1 , # [doc = " A full TLS handshake, with an extra round-trip for a hello retry request."] # [doc = ""] # [doc = " The server can respond with a hello retry request (HRR) if the initial client"] # [doc = " hello is unacceptable for several reasons, the most likely if no supported key"] # [doc = " shares were offered by the client."] FullWithHelloRetryRequest = 0x2 , # [doc = " A resumed TLS handshake."] # [doc = ""] # [doc = " Resumed handshakes involve fewer round trips and less cryptography than"] # [doc = " full ones, but can only happen when the peers have previously done a full"] # [doc = " handshake together, and then remember data about it."] Resumed = 0x3 , }
};
}
