// Generated macro for Stats (struct)
macro_rules! Depcrate_ffiStats {
() => {
// Module: crate::ffi
// Provides: {"Stats"}
// Dependencies: {}
# [repr (C)] pub struct Stats { recv : usize , sent : usize , lost : usize , retrans : usize , sent_bytes : u64 , recv_bytes : u64 , acked_bytes : u64 , lost_bytes : u64 , stream_retrans_bytes : u64 , paths_count : usize , reset_stream_count_local : u64 , stopped_stream_count_local : u64 , reset_stream_count_remote : u64 , stopped_stream_count_remote : u64 , }
};
}
