// Generated macro for CrcReader (struct)
macro_rules! Depcrate_crcCrcReader {
() => {
// Module: crate::crc
// Provides: {"CrcReader"}
// Dependencies: {}
# [doc = " A wrapper around a [`Read`] that calculates the CRC."] # [doc = ""] # [doc = " [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html"] # [derive (Debug)] pub struct CrcReader < R > { inner : R , crc : Crc , }
};
}
