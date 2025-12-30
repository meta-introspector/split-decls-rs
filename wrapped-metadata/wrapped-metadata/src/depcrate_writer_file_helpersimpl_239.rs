// Generated macro for impl_239 (impl)
macro_rules! Depcrate_writer_file_helpersimpl_239 {
() => {
// Module: crate::writer::file::helpers
// Provides: {"impl_239"}
// Dependencies: {}
impl IntoStream for Vec < u8 > { fn into_stream (mut self) -> Self { self . resize (round (self . len () , 4) , 0) ; self } }
};
}
