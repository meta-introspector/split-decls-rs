// Generated macro for impl_134 (impl)
macro_rules! Depcrate_recv_streamimpl_134 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_134"}
// Dependencies: {}
impl From < ReadError > for io :: Error { fn from (x : ReadError) -> Self { use ReadError :: * ; let kind = match x { Reset { .. } | ZeroRttRejected => io :: ErrorKind :: ConnectionReset , ConnectionLost (_) | ClosedStream => io :: ErrorKind :: NotConnected , IllegalOrderedRead => io :: ErrorKind :: InvalidInput , } ; Self :: new (kind , x) } }
};
}
