// Generated macro for impl_42 (impl)
macro_rules! Depcrate_io_writerimpl_42 {
() => {
// Module: crate::io::writer
// Provides: {"impl_42"}
// Dependencies: {}
impl Write for PipeWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { write_to_pipe (self . handle , buf) } fn flush (& mut self) -> io :: Result < () > { flush_pipe (self . handle) } }
};
}
