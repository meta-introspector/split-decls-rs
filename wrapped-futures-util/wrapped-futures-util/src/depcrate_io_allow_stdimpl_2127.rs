// Generated macro for impl_2127 (impl)
macro_rules! Depcrate_io_allow_stdimpl_2127 {
() => {
// Module: crate::io::allow_std
// Provides: {"impl_2127"}
// Dependencies: {}
impl < T > io :: Seek for AllowStdIo < T > where T : io :: Seek , { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . 0 . seek (pos) } }
};
}
