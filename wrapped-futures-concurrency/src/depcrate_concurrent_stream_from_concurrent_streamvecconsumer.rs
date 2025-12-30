// Generated macro for VecConsumer (struct)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamVecConsumer {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"VecConsumer"}
// Dependencies: {}
# [pin_project] pub (crate) struct VecConsumer < 'a , Fut : Future > { # [pin] group : FuturesUnordered < Fut > , output : & 'a mut Vec < Fut :: Output > , }
};
}
