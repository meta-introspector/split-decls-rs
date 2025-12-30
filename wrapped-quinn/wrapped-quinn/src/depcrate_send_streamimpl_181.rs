// Generated macro for impl_181 (impl)
macro_rules! Depcrate_send_streamimpl_181 {
() => {
// Module: crate::send_stream
// Provides: {"impl_181"}
// Dependencies: {}
impl From < StoppedError > for io :: Error { fn from (x : StoppedError) -> Self { use StoppedError :: * ; let kind = match x { ZeroRttRejected => io :: ErrorKind :: ConnectionReset , ConnectionLost (_) => io :: ErrorKind :: NotConnected , } ; Self :: new (kind , x) } }
};
}
