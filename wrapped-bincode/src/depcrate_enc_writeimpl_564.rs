// Generated macro for impl_564 (impl)
macro_rules! Depcrate_enc_writeimpl_564 {
() => {
// Module: crate::enc::write
// Provides: {"impl_564"}
// Dependencies: {}
impl Writer for SizeWriter { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { self . bytes_written += bytes . len () ; Ok (()) } }
};
}
