// Generated macro for impl_2128 (impl)
macro_rules! Depcrate_io_allow_stdimpl_2128 {
() => {
// Module: crate::io::allow_std
// Provides: {"impl_2128"}
// Dependencies: {}
impl < T > AsyncSeek for AllowStdIo < T > where T : io :: Seek , { fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . seek (pos)))) } }
};
}
