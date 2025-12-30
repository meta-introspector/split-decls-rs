// Generated macro for quiche_conn_peer_transport_params (function)
macro_rules! Depcrate_ffiquiche_conn_peer_transport_params {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_peer_transport_params"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_peer_transport_params (conn : & Connection , out : & mut TransportParams ,) -> bool { let tps = match conn . peer_transport_params () { Some (v) => v , None => return false , } ; out . max_idle_timeout = tps . max_idle_timeout ; out . max_udp_payload_size = tps . max_udp_payload_size ; out . initial_max_data = tps . initial_max_data ; out . initial_max_stream_data_bidi_local = tps . initial_max_stream_data_bidi_local ; out . initial_max_stream_data_bidi_remote = tps . initial_max_stream_data_bidi_remote ; out . initial_max_stream_data_uni = tps . initial_max_stream_data_uni ; out . initial_max_streams_bidi = tps . initial_max_streams_bidi ; out . initial_max_streams_uni = tps . initial_max_streams_uni ; out . ack_delay_exponent = tps . ack_delay_exponent ; out . max_ack_delay = tps . max_ack_delay ; out . disable_active_migration = tps . disable_active_migration ; out . active_conn_id_limit = tps . active_conn_id_limit ; out . max_datagram_frame_size = match tps . max_datagram_frame_size { None => Error :: Done . to_c () , Some (v) => v as ssize_t , } ; true }
};
}
