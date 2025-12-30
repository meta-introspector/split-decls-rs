// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl < T : IoSafe + Read > AsyncRead for Async < T > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { loop { match unsafe { (* self) . get_mut () } . read (buf) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_readable (cx)) ? ; } } fn poll_read_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { loop { match unsafe { (* self) . get_mut () } . read_vectored (bufs) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_readable (cx)) ? ; } } }
};
}
