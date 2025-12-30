// Generated macro for impl_1232 (impl)
macro_rules! Depcrate_sslimpl_1232 {
() => {
// Module: crate::ssl
// Provides: {"impl_1232"}
// Dependencies: {}
impl < S : Read + Write > Write for SslStream < S > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { loop { match self . ssl_write (buf) { Ok (n) => return Ok (n) , Err (ref e) if e . code () == ErrorCode :: WANT_READ && e . io_error () . is_none () => { } Err (e) => { return Err (e . into_io_error () . unwrap_or_else (| e | io :: Error :: new (io :: ErrorKind :: Other , e))) ; } } } } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
