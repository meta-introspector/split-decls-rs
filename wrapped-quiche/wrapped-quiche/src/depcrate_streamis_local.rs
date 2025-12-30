// Generated macro for is_local (function)
macro_rules! Depcrate_streamis_local {
() => {
// Module: crate::stream
// Provides: {"is_local"}
// Dependencies: {}
# [doc = " Returns true if the stream was created locally."] pub fn is_local (stream_id : u64 , is_server : bool) -> bool { (stream_id & 0x1) == (is_server as u64) }
};
}
