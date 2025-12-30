// Generated macro for read_be32 (function)
macro_rules! Depcrate_tz_concatenatedread_be32 {
() => {
// Module: crate::tz::concatenated
// Provides: {"read_be32"}
// Dependencies: {}
# [doc = " Reads a 32-bit big endian encoded integer from `bytes`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `bytes.len() != 4`."] fn read_be32 (bytes : & [u8]) -> u32 { u32 :: from_be_bytes (bytes . try_into () . expect ("slice of length 4")) }
};
}
