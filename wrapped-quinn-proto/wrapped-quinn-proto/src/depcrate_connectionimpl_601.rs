// Generated macro for impl_601 (impl)
macro_rules! Depcrate_connectionimpl_601 {
() => {
// Module: crate::connection
// Provides: {"impl_601"}
// Dependencies: {}
impl From < ConnectionError > for io :: Error { fn from (x : ConnectionError) -> Self { use ConnectionError :: * ; let kind = match x { TimedOut => io :: ErrorKind :: TimedOut , Reset => io :: ErrorKind :: ConnectionReset , ApplicationClosed (_) | ConnectionClosed (_) => io :: ErrorKind :: ConnectionAborted , TransportError (_) | VersionMismatch | LocallyClosed | CidsExhausted => { io :: ErrorKind :: Other } } ; Self :: new (kind , x) } }
};
}
