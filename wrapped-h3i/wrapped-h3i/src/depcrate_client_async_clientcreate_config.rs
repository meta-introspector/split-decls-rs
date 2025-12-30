// Generated macro for create_config (function)
macro_rules! Depcrate_client_async_clientcreate_config {
() => {
// Module: crate::client::async_client
// Provides: {"create_config"}
// Dependencies: {}
fn create_config (args : & H3iConfig) -> QuicSettings { let mut quic_settings = QuicSettings :: default () ; quic_settings . verify_peer = args . verify_peer ; quic_settings . max_idle_timeout = Some (Duration :: from_millis (args . idle_timeout)) ; quic_settings . max_recv_udp_payload_size = MAX_DATAGRAM_SIZE ; quic_settings . max_send_udp_payload_size = MAX_DATAGRAM_SIZE ; quic_settings . initial_max_data = 10_000_000 ; quic_settings . initial_max_stream_data_bidi_local = args . max_stream_data_bidi_local ; quic_settings . initial_max_stream_data_bidi_remote = args . max_stream_data_bidi_remote ; quic_settings . initial_max_stream_data_uni = args . max_stream_data_uni ; quic_settings . initial_max_streams_bidi = args . max_streams_bidi ; quic_settings . initial_max_streams_uni = args . max_streams_uni ; quic_settings . disable_active_migration = true ; quic_settings . active_connection_id_limit = 0 ; quic_settings . max_connection_window = args . max_window ; quic_settings . max_stream_window = args . max_stream_window ; quic_settings . grease = false ; quic_settings . capture_quiche_logs = true ; quic_settings . keylog_file = std :: env :: var_os ("SSLKEYLOGFILE") . and_then (| os_str | os_str . into_string () . ok ()) ; quic_settings . enable_dgram = args . enable_dgram ; quic_settings . dgram_recv_max_queue_len = args . dgram_recv_queue_len ; quic_settings . dgram_send_max_queue_len = args . dgram_send_queue_len ; quic_settings }
};
}
