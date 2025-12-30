// Generated macro for create_config (function)
macro_rules! Depcrate_client_sync_clientcreate_config {
() => {
// Module: crate::client::sync_client
// Provides: {"create_config"}
// Dependencies: {}
fn create_config (args : & Config , should_log_keys : bool) -> quiche :: Config { let mut config = quiche :: Config :: new (QUIC_VERSION) . unwrap () ; config . verify_peer (args . verify_peer) ; config . set_application_protos (& [b"h3"]) . unwrap () ; config . set_max_idle_timeout (args . idle_timeout) ; config . set_max_recv_udp_payload_size (MAX_DATAGRAM_SIZE) ; config . set_max_send_udp_payload_size (MAX_DATAGRAM_SIZE) ; config . set_initial_max_data (10_000_000) ; config . set_initial_max_stream_data_bidi_local (args . max_stream_data_bidi_local) ; config . set_initial_max_stream_data_bidi_remote (args . max_stream_data_bidi_remote ,) ; config . set_initial_max_stream_data_uni (args . max_stream_data_uni) ; config . set_initial_max_streams_bidi (args . max_streams_bidi) ; config . set_initial_max_streams_uni (args . max_streams_uni) ; config . set_disable_active_migration (true) ; config . set_active_connection_id_limit (0) ; config . set_max_connection_window (args . max_window) ; config . set_max_stream_window (args . max_stream_window) ; config . grease (false) ; if args . enable_dgram { config . enable_dgram (true , args . dgram_recv_queue_len , args . dgram_send_queue_len ,) ; } if should_log_keys { config . log_keys () } config }
};
}
