// Generated macro for impl_326 (impl)
macro_rules! Depcrate_stream_logimpl_326 {
() => {
// Module: crate::stream::log
// Provides: {"impl_326"}
// Dependencies: {}
impl < S : Read , W : Write > Read for LogStream < S , W > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { let n = self . stream . read (buf) ? ; self . log_read (& buf [.. n]) ; Ok (n) } }
};
}
