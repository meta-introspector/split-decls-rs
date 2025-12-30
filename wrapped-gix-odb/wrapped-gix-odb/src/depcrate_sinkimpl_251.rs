// Generated macro for impl_251 (impl)
macro_rules! Depcrate_sinkimpl_251 {
() => {
// Module: crate::sink
// Provides: {"impl_251"}
// Dependencies: {}
impl Sink { # [doc = " Enable or disable compression. Compression is disabled by default"] pub fn compress (mut self , enable : bool) -> Self { if enable { self . compressor = Some (RefCell :: new (deflate :: Write :: new (io :: sink ()))) ; } else { self . compressor = None ; } self } }
};
}
