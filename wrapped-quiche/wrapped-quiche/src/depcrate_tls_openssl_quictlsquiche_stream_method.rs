// Generated macro for QUICHE_STREAM_METHOD (static)
macro_rules! Depcrate_tls_openssl_quictlsQUICHE_STREAM_METHOD {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"QUICHE_STREAM_METHOD"}
// Dependencies: {}
pub (super) static QUICHE_STREAM_METHOD : SSL_QUIC_METHOD = SSL_QUIC_METHOD { set_encryption_secrets : Some (set_encryption_secrets) , add_handshake_data : Some (add_handshake_data) , flush_flight : Some (flush_flight) , send_alert : Some (send_alert) , } ;
};
}
