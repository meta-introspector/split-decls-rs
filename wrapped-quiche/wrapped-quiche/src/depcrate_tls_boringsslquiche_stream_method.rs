// Generated macro for QUICHE_STREAM_METHOD (static)
macro_rules! Depcrate_tls_boringsslQUICHE_STREAM_METHOD {
() => {
// Module: crate::tls::boringssl
// Provides: {"QUICHE_STREAM_METHOD"}
// Dependencies: {}
pub (super) static QUICHE_STREAM_METHOD : SSL_QUIC_METHOD = SSL_QUIC_METHOD { set_read_secret : Some (set_read_secret) , set_write_secret : Some (set_write_secret) , add_handshake_data : Some (add_handshake_data) , flush_flight : Some (flush_flight) , send_alert : Some (send_alert) , } ;
};
}
