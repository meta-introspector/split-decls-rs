// Generated macro for impl_1024 (impl)
macro_rules! Depcrate_ssl_errorimpl_1024 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1024"}
// Dependencies: {}
impl Error { pub fn code (& self) -> ErrorCode { self . code } pub fn io_error (& self) -> Option < & io :: Error > { match self . cause { Some (InnerError :: Io (ref e)) => Some (e) , _ => None , } } pub fn into_io_error (self) -> Result < io :: Error , Error > { match self . cause { Some (InnerError :: Io (e)) => Ok (e) , _ => Err (self) , } } pub fn ssl_error (& self) -> Option < & ErrorStack > { match self . cause { Some (InnerError :: Ssl (ref e)) => Some (e) , _ => None , } } }
};
}
