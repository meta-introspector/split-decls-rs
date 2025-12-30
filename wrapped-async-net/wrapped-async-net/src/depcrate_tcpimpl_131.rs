// Generated macro for impl_131 (impl)
macro_rules! Depcrate_tcpimpl_131 {
() => {
// Module: crate::tcp
// Provides: {"impl_131"}
// Dependencies: {}
impl AsyncRead for TcpStream { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { loop { match self . inner . get_ref () . read (buf) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => { self . readable = None ; return Poll :: Ready (res) ; } } if self . readable . is_none () { self . readable = Some (self . inner . clone () . readable_owned ()) ; } if let Some (f) = & mut self . readable { let res = ready ! (Pin :: new (f) . poll (cx)) ; self . readable = None ; res ? ; } } } }
};
}
