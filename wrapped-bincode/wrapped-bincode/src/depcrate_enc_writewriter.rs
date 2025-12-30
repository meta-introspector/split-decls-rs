// Generated macro for Writer (trait)
macro_rules! Depcrate_enc_writeWriter {
() => {
// Module: crate::enc::write
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " Trait that indicates that a struct can be used as a destination to encode data too. This is used by [Encode]"] # [doc = ""] # [doc = " [Encode]: ../trait.Encode.html"] pub trait Writer { # [doc = " Write `bytes` to the underlying writer. Exactly `bytes.len()` bytes must be written, or else an error should be returned."] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > ; }
};
}
