// Generated macro for from_be_bytes_i64 (function)
macro_rules! Depcrate_shared_tziffrom_be_bytes_i64 {
() => {
// Module: crate::shared::tzif
// Provides: {"from_be_bytes_i64"}
// Dependencies: {}
# [doc = " Interprets the given slice as a signed 64-bit big endian integer and"] # [doc = " returns it."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When `bytes.len() != 8`."] fn from_be_bytes_i64 (bytes : & [u8]) -> i64 { i64 :: from_be_bytes (bytes . try_into () . unwrap ()) }
};
}
