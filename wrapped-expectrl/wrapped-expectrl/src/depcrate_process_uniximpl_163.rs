// Generated macro for impl_163 (impl)
macro_rules! Depcrate_process_uniximpl_163 {
() => {
// Module: crate::process::unix
// Provides: {"impl_163"}
// Dependencies: {}
impl Write for PtyStream { fn write (& mut self , buf : & [u8]) -> Result < usize > { self . handle . write (buf) } fn flush (& mut self) -> Result < () > { self . handle . flush () } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> Result < usize > { self . handle . write_vectored (bufs) } }
};
}
