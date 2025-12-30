// Generated macro for quiche_conn_path_stats (function)
macro_rules! Depcrate_ffiquiche_conn_path_stats {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_path_stats"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_path_stats (conn : & Connection , idx : usize , out : & mut PathStats ,) -> c_int { let stats = match conn . path_stats () . nth (idx) { Some (p) => p , None => return Error :: Done . to_c () as c_int , } ; out . local_addr_len = std_addr_to_c (& stats . local_addr , & mut out . local_addr) ; out . peer_addr_len = std_addr_to_c (& stats . peer_addr , & mut out . peer_addr) ; out . validation_state = stats . validation_state . to_c () ; out . active = stats . active ; out . recv = stats . recv ; out . sent = stats . sent ; out . lost = stats . lost ; out . retrans = stats . retrans ; out . rtt = stats . rtt . as_nanos () as u64 ; out . min_rtt = stats . min_rtt . unwrap_or_default () . as_nanos () as u64 ; out . rttvar = stats . rttvar . as_nanos () as u64 ; out . cwnd = stats . cwnd ; out . sent_bytes = stats . sent_bytes ; out . recv_bytes = stats . recv_bytes ; out . lost_bytes = stats . lost_bytes ; out . stream_retrans_bytes = stats . stream_retrans_bytes ; out . pmtu = stats . pmtu ; out . delivery_rate = stats . delivery_rate ; 0 }
};
}
