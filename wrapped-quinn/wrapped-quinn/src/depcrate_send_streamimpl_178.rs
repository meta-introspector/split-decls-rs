// Generated macro for impl_178 (impl)
macro_rules! Depcrate_send_streamimpl_178 {
() => {
// Module: crate::send_stream
// Provides: {"impl_178"}
// Dependencies: {}
impl From < StoppedError > for WriteError { fn from (x : StoppedError) -> Self { match x { StoppedError :: ConnectionLost (e) => Self :: ConnectionLost (e) , StoppedError :: ZeroRttRejected => Self :: ZeroRttRejected , } } }
};
}
