// Generated macro for write_panic (function)
macro_rules! Depcrate_ssl_testwrite_panic {
() => {
// Module: crate::ssl::test
// Provides: {"write_panic"}
// Dependencies: {}
# [test] # [should_panic (expected = "blammo")] fn write_panic () { struct ExplodingStream (TcpStream) ; impl Read for ExplodingStream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } } impl Write for ExplodingStream { fn write (& mut self , _ : & [u8]) -> io :: Result < usize > { panic ! ("blammo") ; } fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } } let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let stream = ExplodingStream (server . connect_tcp ()) ; let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; let _ = Ssl :: new (& ctx . build ()) . unwrap () . connect (stream) ; }
};
}
