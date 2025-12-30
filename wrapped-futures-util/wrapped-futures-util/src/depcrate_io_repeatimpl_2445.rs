// Generated macro for impl_2445 (impl)
macro_rules! Depcrate_io_repeatimpl_2445 {
() => {
// Module: crate::io::repeat
// Provides: {"impl_2445"}
// Dependencies: {}
impl AsyncRead for Repeat { # [inline] fn poll_read (self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { for slot in & mut * buf { * slot = self . byte ; } Poll :: Ready (Ok (buf . len ())) } # [inline] fn poll_read_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { let mut nwritten = 0 ; for buf in bufs { nwritten += ready ! (self . as_mut () . poll_read (cx , buf)) ? ; } Poll :: Ready (Ok (nwritten)) } }
};
}
