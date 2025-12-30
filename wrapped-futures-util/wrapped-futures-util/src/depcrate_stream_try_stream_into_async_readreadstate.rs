// Generated macro for ReadState (enum)
macro_rules! Depcrate_stream_try_stream_into_async_readReadState {
() => {
// Module: crate::stream::try_stream::into_async_read
// Provides: {"ReadState"}
// Dependencies: {}
# [derive (Debug)] enum ReadState < T : AsRef < [u8] > > { Ready { chunk : T , chunk_start : usize } , PendingChunk , Eof , }
};
}
