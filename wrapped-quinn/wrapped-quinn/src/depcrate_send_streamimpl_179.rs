// Generated macro for impl_179 (impl)
macro_rules! Depcrate_send_streamimpl_179 {
() => {
// Module: crate::send_stream
// Provides: {"impl_179"}
// Dependencies: {}
impl From < WriteError > for io :: Error { fn from (x : WriteError) -> Self { use WriteError :: * ; let kind = match x { Stopped (_) | ZeroRttRejected => io :: ErrorKind :: ConnectionReset , ConnectionLost (_) | ClosedStream => io :: ErrorKind :: NotConnected , } ; Self :: new (kind , x) } }
};
}
