// Generated macro for quiche_path_event_reused_source_connection_id (function)
macro_rules! Depcrate_ffiquiche_path_event_reused_source_connection_id {
() => {
// Module: crate::ffi
// Provides: {"quiche_path_event_reused_source_connection_id"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_path_event_reused_source_connection_id (ev : & PathEvent , cid_sequence_number : & mut u64 , old_local_addr : & mut sockaddr_storage , old_local_addr_len : & mut socklen_t , old_peer_addr : & mut sockaddr_storage , old_peer_addr_len : & mut socklen_t , local_addr : & mut sockaddr_storage , local_addr_len : & mut socklen_t , peer_addr : & mut sockaddr_storage , peer_addr_len : & mut socklen_t ,) { match ev { PathEvent :: ReusedSourceConnectionId (id , old , new) => { * cid_sequence_number = * id ; * old_local_addr_len = std_addr_to_c (& old . 0 , old_local_addr) ; * old_peer_addr_len = std_addr_to_c (& old . 1 , old_peer_addr) ; * local_addr_len = std_addr_to_c (& new . 0 , local_addr) ; * peer_addr_len = std_addr_to_c (& new . 1 , peer_addr) } , _ => unreachable ! () , } }
};
}
