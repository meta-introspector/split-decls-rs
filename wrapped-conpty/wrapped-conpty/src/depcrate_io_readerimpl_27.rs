// Generated macro for impl_27 (impl)
macro_rules! Depcrate_io_readerimpl_27 {
() => {
// Module: crate::io::reader
// Provides: {"impl_27"}
// Dependencies: {}
impl Read for PipeReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { read_pipe (self . handle , buf , self . blocking) } }
};
}
