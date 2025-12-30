// Generated macro for read_u128 (function)
macro_rules! Depcrate_util_wireread_u128 {
() => {
// Module: crate::util::wire
// Provides: {"read_u128"}
// Dependencies: {}
# [doc = " Read a u128 from the beginning of the given slice in native endian format."] # [doc = " If the slice has fewer than 16 bytes, then this panics."] pub (crate) fn read_u128 (slice : & [u8]) -> u128 { let bytes : [u8 ; 16] = slice [.. size_of :: < u128 > ()] . try_into () . unwrap () ; u128 :: from_ne_bytes (bytes) }
};
}
