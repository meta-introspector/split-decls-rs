// Generated macro for TryReadyChunksError (struct)
macro_rules! Depcrate_stream_try_stream_try_ready_chunksTryReadyChunksError {
() => {
// Module: crate::stream::try_stream::try_ready_chunks
// Provides: {"TryReadyChunksError"}
// Dependencies: {}
# [doc = " Error indicating, that while chunk was collected inner stream produced an error."] # [doc = ""] # [doc = " Contains all items that were collected before an error occurred, and the stream error itself."] # [derive (PartialEq , Eq)] pub struct TryReadyChunksError < T , E > (pub Vec < T > , pub E) ;
};
}
