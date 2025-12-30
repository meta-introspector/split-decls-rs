// Generated macro for impl_132 (impl)
macro_rules! Depcrate_recv_streamimpl_132 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_132"}
// Dependencies: {}
impl From < ReadableError > for ReadError { fn from (e : ReadableError) -> Self { match e { ReadableError :: ClosedStream => Self :: ClosedStream , ReadableError :: IllegalOrderedRead => Self :: IllegalOrderedRead , } } }
};
}
