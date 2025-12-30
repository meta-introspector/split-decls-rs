// Generated macro for Handshake (struct)
macro_rules! Depcrate_tlsHandshake {
() => {
// Module: crate::tls
// Provides: {"Handshake"}
// Dependencies: {}
pub struct Handshake { # [doc = " Raw pointer"] ptr : * mut SSL , # [doc = " SSL_process_quic_post_handshake should be called when whenever"] # [doc = " SSL_provide_quic_data is called to process the provided data."] provided_data_outstanding : bool , }
};
}
