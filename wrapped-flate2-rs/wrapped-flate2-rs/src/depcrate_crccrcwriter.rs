// Generated macro for CrcWriter (struct)
macro_rules! Depcrate_crcCrcWriter {
() => {
// Module: crate::crc
// Provides: {"CrcWriter"}
// Dependencies: {}
# [doc = " A wrapper around a [`Write`] that calculates the CRC."] # [doc = ""] # [doc = " [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"] # [derive (Debug)] pub struct CrcWriter < W > { inner : W , crc : Crc , }
};
}
