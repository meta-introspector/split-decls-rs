// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_ssl_errorimpl_1027 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1027"}
// Dependencies: {}
impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self . cause { Some (InnerError :: Io (ref e)) => Some (e) , Some (InnerError :: Ssl (ref e)) => Some (e) , None => None , } } }
};
}
