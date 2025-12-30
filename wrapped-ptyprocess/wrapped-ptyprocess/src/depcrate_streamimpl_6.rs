// Generated macro for impl_6 (impl)
macro_rules! Depcrate_streamimpl_6 {
() => {
// Module: crate::stream
// Provides: {"impl_6"}
// Dependencies: {}
impl Read for Stream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match self . inner . read (buf) { Err (ref err) if has_reached_end_of_sdtout (err) => Ok (0) , result => result , } } }
};
}
