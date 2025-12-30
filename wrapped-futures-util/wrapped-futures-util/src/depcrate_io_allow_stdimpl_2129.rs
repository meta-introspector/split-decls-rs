// Generated macro for impl_2129 (impl)
macro_rules! Depcrate_io_allow_stdimpl_2129 {
() => {
// Module: crate::io::allow_std
// Provides: {"impl_2129"}
// Dependencies: {}
impl < T > io :: BufRead for AllowStdIo < T > where T : io :: BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . 0 . fill_buf () } fn consume (& mut self , amt : usize) { self . 0 . consume (amt) } }
};
}
