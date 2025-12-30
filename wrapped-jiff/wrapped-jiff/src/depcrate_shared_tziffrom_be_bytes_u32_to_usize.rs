// Generated macro for from_be_bytes_u32_to_usize (function)
macro_rules! Depcrate_shared_tziffrom_be_bytes_u32_to_usize {
() => {
// Module: crate::shared::tzif
// Provides: {"from_be_bytes_u32_to_usize"}
// Dependencies: {}
# [doc = " Interprets the given slice as an unsigned 32-bit big endian integer,"] # [doc = " attempts to convert it to a `usize` and returns it."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When `bytes.len() != 4`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This errors if the `u32` parsed from the given bytes cannot fit in a"] # [doc = " `usize`."] fn from_be_bytes_u32_to_usize (bytes : & [u8]) -> Result < usize , Error > { let n = from_be_bytes_u32 (bytes) ; usize :: try_from (n) . map_err (| _ | { err ! ("failed to parse integer {n} (too big, max allowed is {}" , usize :: MAX) }) }
};
}
