// Generated macro for impl_327 (impl)
macro_rules! Depcrate_stream_logimpl_327 {
() => {
// Module: crate::stream::log
// Provides: {"impl_327"}
// Dependencies: {}
impl < S , W > NonBlocking for LogStream < S , W > where S : NonBlocking , { fn set_blocking (& mut self , on : bool) -> Result < () > { self . stream . set_blocking (on) } }
};
}
