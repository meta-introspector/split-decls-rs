// Generated macro for quiche_path_event_failed_validation (function)
macro_rules! Depcrate_ffiquiche_path_event_failed_validation {
() => {
// Module: crate::ffi
// Provides: {"quiche_path_event_failed_validation"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_path_event_failed_validation (ev : & PathEvent , local_addr : & mut sockaddr_storage , local_addr_len : & mut socklen_t , peer_addr : & mut sockaddr_storage , peer_addr_len : & mut socklen_t ,) { match ev { PathEvent :: FailedValidation (local , peer) => { * local_addr_len = std_addr_to_c (local , local_addr) ; * peer_addr_len = std_addr_to_c (peer , peer_addr) } , _ => unreachable ! () , } }
};
}
