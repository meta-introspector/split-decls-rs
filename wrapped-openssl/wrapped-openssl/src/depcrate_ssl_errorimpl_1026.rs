// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_ssl_errorimpl_1026 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1026"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . code { ErrorCode :: ZERO_RETURN => fmt . write_str ("the SSL session has been shut down") , ErrorCode :: WANT_READ => match self . io_error () { Some (_) => fmt . write_str ("a nonblocking read call would have blocked") , None => fmt . write_str ("the operation should be retried") , } , ErrorCode :: WANT_WRITE => match self . io_error () { Some (_) => fmt . write_str ("a nonblocking write call would have blocked") , None => fmt . write_str ("the operation should be retried") , } , ErrorCode :: SYSCALL => match self . io_error () { Some (err) => write ! (fmt , "{}" , err) , None => fmt . write_str ("unexpected EOF") , } , ErrorCode :: SSL => match self . ssl_error () { Some (e) => write ! (fmt , "{}" , e) , None => fmt . write_str ("OpenSSL error") , } , ErrorCode (code) => write ! (fmt , "unknown error code {}" , code) , } } }
};
}
