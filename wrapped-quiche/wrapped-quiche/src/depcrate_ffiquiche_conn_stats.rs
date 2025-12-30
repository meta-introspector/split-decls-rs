// Generated macro for quiche_conn_stats (function)
macro_rules! Depcrate_ffiquiche_conn_stats {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stats"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stats (conn : & Connection , out : & mut Stats) { let stats = conn . stats () ; out . recv = stats . recv ; out . sent = stats . sent ; out . lost = stats . lost ; out . retrans = stats . retrans ; out . sent_bytes = stats . sent_bytes ; out . recv_bytes = stats . recv_bytes ; out . acked_bytes = stats . acked_bytes ; out . lost_bytes = stats . lost_bytes ; out . stream_retrans_bytes = stats . stream_retrans_bytes ; out . paths_count = stats . paths_count ; out . reset_stream_count_local = stats . reset_stream_count_local ; out . stopped_stream_count_local = stats . stopped_stream_count_local ; out . reset_stream_count_remote = stats . reset_stream_count_remote ; out . stopped_stream_count_remote = stats . stopped_stream_count_remote ; }
};
}
