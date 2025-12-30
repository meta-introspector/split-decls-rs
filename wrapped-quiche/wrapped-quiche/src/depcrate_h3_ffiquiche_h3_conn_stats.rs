// Generated macro for quiche_h3_conn_stats (function)
macro_rules! Depcrate_h3_ffiquiche_h3_conn_stats {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_conn_stats"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_conn_stats (conn : & h3 :: Connection , out : & mut Stats) { let stats = conn . stats () ; out . qpack_encoder_stream_recv_bytes = stats . qpack_encoder_stream_recv_bytes ; out . qpack_decoder_stream_recv_bytes = stats . qpack_decoder_stream_recv_bytes ; }
};
}
