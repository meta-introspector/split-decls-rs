// Generated macro for impl_545 (impl)
macro_rules! Depcrate_server_conn_autoimpl_545 {
() => {
// Module: crate::server::conn::auto
// Provides: {"impl_545"}
// Dependencies: {}
impl Version { # [must_use] # [cfg (any (not (feature = "http2") , not (feature = "http1")))] pub fn unsupported (self) -> Error { match self { Version :: H1 => Error :: from ("HTTP/1 is not supported") , Version :: H2 => Error :: from ("HTTP/2 is not supported") , } } }
};
}
