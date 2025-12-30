// Generated macro for StreamStats (struct)
macro_rules! Depcrate_statsStreamStats {
() => {
// Module: crate::stats
// Provides: {"StreamStats"}
// Dependencies: {}
pub struct StreamStats { id : StreamId , request_size : u64 , bytes : AtomicUsize , sender : bool , finished : AtomicBool , duration : AtomicU64 , first_byte_latency : AtomicU64 , }
};
}
