// Generated macro for CombineSyncRead (trait)
macro_rules! Depcrate_stream_buf_readerCombineSyncRead {
() => {
// Module: crate::stream::buf_reader
// Provides: {"CombineSyncRead"}
// Dependencies: {}
# [doc (hidden)] pub trait CombineSyncRead < R > : CombineBuffer < R > { fn extend_buf_sync (& mut self , read : & mut R) -> io :: Result < usize > ; }
};
}
