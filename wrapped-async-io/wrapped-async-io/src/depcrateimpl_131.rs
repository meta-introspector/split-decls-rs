// Generated macro for impl_131 (impl)
macro_rules! Depcrateimpl_131 {
() => {
// Module: crate
// Provides: {"impl_131"}
// Dependencies: {}
impl < T : IoSafe + Write > AsyncWrite for Async < T > { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { loop { match unsafe { (* self) . get_mut () } . write (buf) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_writable (cx)) ? ; } } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { loop { match unsafe { (* self) . get_mut () } . write_vectored (bufs) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_writable (cx)) ? ; } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { loop { match unsafe { (* self) . get_mut () } . flush () { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_writable (cx)) ? ; } } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_flush (cx) } }
};
}
