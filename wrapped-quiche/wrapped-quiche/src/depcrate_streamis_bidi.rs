// Generated macro for is_bidi (function)
macro_rules! Depcrate_streamis_bidi {
() => {
// Module: crate::stream
// Provides: {"is_bidi"}
// Dependencies: {}
# [doc = " Returns true if the stream is bidirectional."] pub fn is_bidi (stream_id : u64) -> bool { (stream_id & 0x2) == 0 }
};
}
