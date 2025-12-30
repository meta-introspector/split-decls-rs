// Generated macro for impl_578 (impl)
macro_rules! Depcrate_connection_streamsimpl_578 {
() => {
// Module: crate::connection::streams
// Provides: {"impl_578"}
// Dependencies: {}
impl From < ClosedStream > for io :: Error { fn from (x : ClosedStream) -> Self { Self :: new (io :: ErrorKind :: NotConnected , x) } }
};
}
