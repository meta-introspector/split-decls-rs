// Generated macro for impl_2125 (impl)
macro_rules! Depcrate_io_allow_stdimpl_2125 {
() => {
// Module: crate::io::allow_std
// Provides: {"impl_2125"}
// Dependencies: {}
impl < T > io :: Read for AllowStdIo < T > where T : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . 0 . read_to_end (buf) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { self . 0 . read_to_string (buf) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . 0 . read_exact (buf) } }
};
}
