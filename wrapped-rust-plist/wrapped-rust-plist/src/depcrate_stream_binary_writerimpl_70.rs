// Generated macro for impl_70 (impl)
macro_rules! Depcrate_stream_binary_writerimpl_70 {
() => {
// Module: crate::stream::binary_writer
// Provides: {"impl_70"}
// Dependencies: {}
impl < W : Write > PosWriter < W > { fn write_exact (& mut self , buf : & [u8]) -> Result < () , Error > { self . write_all (buf) . map_err (error :: from_io_without_position) ? ; Ok (()) } }
};
}
