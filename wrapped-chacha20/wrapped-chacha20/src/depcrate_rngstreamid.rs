// Generated macro for StreamId (type)
macro_rules! Depcrate_rngStreamId {
() => {
// Module: crate::rng
// Provides: {"StreamId"}
// Dependencies: {}
# [doc = " A wrapper for `stream_id`."] # [doc = ""] # [doc = " Can be constructed from any of the following:"] # [doc = " * `u64`"] # [doc = " * `[u32; 2]`"] # [doc = " * `[u8; 8]`"] # [doc = ""] # [doc = " The arrays should be in little endian order."] pub type StreamId = U32x2 ;
};
}
