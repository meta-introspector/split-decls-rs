// Generated macro for impl_41 (impl)
macro_rules! Depcrate_io_writerimpl_41 {
() => {
// Module: crate::io::writer
// Provides: {"impl_41"}
// Dependencies: {}
impl PipeWriter { # [doc = " Creates a new instance of PipeWriter."] # [doc = ""] # [doc = " It owns a HANDLE."] pub fn new (handle : HANDLE) -> Self { Self { handle } } # [doc = " Tries to make a clone of PipeWriter."] pub fn try_clone (& self) -> Result < Self , Error > { clone_handle (self . handle) . map_err (Into :: into) . map (Self :: new) } }
};
}
