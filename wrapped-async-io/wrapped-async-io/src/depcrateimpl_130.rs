// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl < T > AsyncRead for & Async < T > where for < 'a > & 'a T : Read , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { loop { match (* self) . get_ref () . read (buf) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_readable (cx)) ? ; } } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { loop { match (* self) . get_ref () . read_vectored (bufs) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => return Poll :: Ready (res) , } ready ! (self . poll_readable (cx)) ? ; } } }
};
}
