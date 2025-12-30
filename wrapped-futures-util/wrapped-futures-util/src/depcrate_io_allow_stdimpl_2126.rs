// Generated macro for impl_2126 (impl)
macro_rules! Depcrate_io_allow_stdimpl_2126 {
() => {
// Module: crate::io::allow_std
// Provides: {"impl_2126"}
// Dependencies: {}
impl < T > AsyncRead for AllowStdIo < T > where T : io :: Read , { fn poll_read (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . read (buf)))) } fn poll_read_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . read_vectored (bufs)))) } }
};
}
