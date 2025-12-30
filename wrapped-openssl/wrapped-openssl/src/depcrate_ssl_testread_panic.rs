// Generated macro for read_panic (function)
macro_rules! Depcrate_ssl_testread_panic {
() => {
// Module: crate::ssl::test
// Provides: {"read_panic"}
// Dependencies: {}
# [test] # [should_panic (expected = "blammo")] fn read_panic () { struct ExplodingStream (TcpStream) ; impl Read for ExplodingStream { fn read (& mut self , _ : & mut [u8]) -> io :: Result < usize > { panic ! ("blammo") ; } } impl Write for ExplodingStream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } } let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let stream = ExplodingStream (server . connect_tcp ()) ; let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; let _ = Ssl :: new (& ctx . build ()) . unwrap () . connect (stream) ; }
};
}
