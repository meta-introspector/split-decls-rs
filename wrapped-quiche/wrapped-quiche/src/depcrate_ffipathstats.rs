// Generated macro for PathStats (struct)
macro_rules! Depcrate_ffiPathStats {
() => {
// Module: crate::ffi
// Provides: {"PathStats"}
// Dependencies: {}
# [repr (C)] pub struct PathStats { local_addr : sockaddr_storage , local_addr_len : socklen_t , peer_addr : sockaddr_storage , peer_addr_len : socklen_t , validation_state : ssize_t , active : bool , recv : usize , sent : usize , lost : usize , retrans : usize , rtt : u64 , min_rtt : u64 , rttvar : u64 , cwnd : usize , sent_bytes : u64 , recv_bytes : u64 , lost_bytes : u64 , stream_retrans_bytes : u64 , pmtu : usize , delivery_rate : u64 , }
};
}
