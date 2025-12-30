// Generated macro for impl_1481 (impl)
macro_rules! Depcrate_stream_try_stream_into_async_readimpl_1481 {
() => {
// Module: crate::stream::try_stream::into_async_read
// Provides: {"impl_1481"}
// Dependencies: {}
impl < St > IntoAsyncRead < St > where St : TryStream < Error = Error > , St :: Ok : AsRef < [u8] > , { pub (super) fn new (stream : St) -> Self { Self { stream , state : ReadState :: PendingChunk } } }
};
}
