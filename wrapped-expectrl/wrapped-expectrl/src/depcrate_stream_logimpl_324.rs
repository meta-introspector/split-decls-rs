// Generated macro for impl_324 (impl)
macro_rules! Depcrate_stream_logimpl_324 {
() => {
// Module: crate::stream::log
// Provides: {"impl_324"}
// Dependencies: {}
impl < S , W : Write > LogStream < S , W > { fn log_write (& mut self , buf : & [u8]) { log (& mut self . logger , "write" , buf) ; } fn log_read (& mut self , buf : & [u8]) { log (& mut self . logger , "read" , buf) ; } }
};
}
