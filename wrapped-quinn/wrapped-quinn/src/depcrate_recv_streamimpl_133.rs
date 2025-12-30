// Generated macro for impl_133 (impl)
macro_rules! Depcrate_recv_streamimpl_133 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_133"}
// Dependencies: {}
impl From < ResetError > for ReadError { fn from (e : ResetError) -> Self { match e { ResetError :: ConnectionLost (e) => Self :: ConnectionLost (e) , ResetError :: ZeroRttRejected => Self :: ZeroRttRejected , } } }
};
}
