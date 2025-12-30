// Generated macro for impl_136 (impl)
macro_rules! Depcrate_recv_streamimpl_136 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_136"}
// Dependencies: {}
impl From < ResetError > for io :: Error { fn from (x : ResetError) -> Self { use ResetError :: * ; let kind = match x { ZeroRttRejected => io :: ErrorKind :: ConnectionReset , ConnectionLost (_) => io :: ErrorKind :: NotConnected , } ; Self :: new (kind , x) } }
};
}
