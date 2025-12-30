// Generated macro for ResultVecConsumer (struct)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamResultVecConsumer {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"ResultVecConsumer"}
// Dependencies: {}
# [pin_project] pub (crate) struct ResultVecConsumer < 'a , Fut : Future , T , E > { # [pin] group : FuturesUnordered < Fut > , output : & 'a mut Result < Vec < T > , E > , }
};
}
