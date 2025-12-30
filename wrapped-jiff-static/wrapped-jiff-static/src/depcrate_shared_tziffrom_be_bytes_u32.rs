// Generated macro for from_be_bytes_u32 (function)
macro_rules! Depcrate_shared_tziffrom_be_bytes_u32 {
() => {
// Module: crate::shared::tzif
// Provides: {"from_be_bytes_u32"}
// Dependencies: {}
# [doc = " Interprets the given slice as an unsigned 32-bit big endian integer and"] # [doc = " returns it."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When `bytes.len() != 4`."] fn from_be_bytes_u32 (bytes : & [u8]) -> u32 { u32 :: from_be_bytes (bytes . try_into () . unwrap ()) }
};
}
