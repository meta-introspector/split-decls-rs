// Generated macro for macro_1479 (macro)
macro_rules! Depcrate_stream_try_stream_into_async_readmacro_1479 {
() => {
// Module: crate::stream::try_stream::into_async_read
// Provides: {"macro_1479"}
// Dependencies: {}
pin_project ! { # [doc = " Reader for the [`into_async_read`](super::TryStreamExt::into_async_read) method."] # [derive (Debug)] # [must_use = "readers do nothing unless polled"] # [cfg_attr (docsrs , doc (cfg (feature = "io")))] pub struct IntoAsyncRead < St > where St : TryStream < Error = Error >, St :: Ok : AsRef < [u8] >, { # [pin] stream : St , state : ReadState < St :: Ok >, } }
};
}
