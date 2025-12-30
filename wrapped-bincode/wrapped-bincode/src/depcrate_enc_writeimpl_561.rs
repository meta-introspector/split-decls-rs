// Generated macro for impl_561 (impl)
macro_rules! Depcrate_enc_writeimpl_561 {
() => {
// Module: crate::enc::write
// Provides: {"impl_561"}
// Dependencies: {}
impl < 'storage > SliceWriter < 'storage > { # [doc = " Create a new instance of `SliceWriter` with the given byte array."] pub fn new (bytes : & 'storage mut [u8]) -> SliceWriter < 'storage > { let original = bytes . len () ; SliceWriter { slice : bytes , original_length : original , } } # [doc = " Return the amount of bytes written so far."] pub fn bytes_written (& self) -> usize { self . original_length - self . slice . len () } }
};
}
