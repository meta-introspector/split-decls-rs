// Generated macro for impl_132 (impl)
macro_rules! Depcrateimpl_132 {
() => {
// Module: crate
// Provides: {"impl_132"}
// Dependencies: {}
impl < T > AsyncWrite for & Async < T > where for < 'a > & 'a T : Write , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { loop { match (* self) . get_ref () . write (buf) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_writable (cx)) ? ; } } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { loop { match (* self) . get_ref () . write_vectored (bufs) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_writable (cx)) ? ; } } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { loop { match (* self) . get_ref () . flush () { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_writable (cx)) ? ; } } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_flush (cx) } }
};
}
