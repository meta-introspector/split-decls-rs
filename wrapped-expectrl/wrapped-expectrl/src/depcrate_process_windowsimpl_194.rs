// Generated macro for impl_194 (impl)
macro_rules! Depcrate_process_windowsimpl_194 {
() => {
// Module: crate::process::windows
// Provides: {"impl_194"}
// Dependencies: {}
impl Write for ProcessStream { fn write (& mut self , buf : & [u8]) -> Result < usize > { self . input . write (buf) } fn flush (& mut self) -> Result < () > { self . input . flush () } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> Result < usize > { self . input . write_vectored (bufs) } }
};
}
