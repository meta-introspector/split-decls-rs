// Generated macro for decode_hex (function)
macro_rules! Depcrate_utildecode_hex {
() => {
// Module: crate::util
// Provides: {"decode_hex"}
// Dependencies: {}
# [doc = " # Errors"] # [doc = " Returns an error when"] # [doc = " - `s` is not twice the length of `N`"] # [doc = " - the string contains non-hex symbols"] pub fn decode_hex < const N : usize > (s : & str) -> Result < [u8 ; N] , String > { if s . len () != N * 2 { return Err (format ! ("expected length {} chars, got {} chars" , N * 2 , s . len ())) ; } let mut result = [0u8 ; N] ; # [allow (clippy :: needless_range_loop)] for x in 0 .. N { let idx = 2 * x ; result [x] = u8 :: from_str_radix (& s [idx .. idx + 2] , 16) . map_err (| _ | "string contains non-hex symbols") ? ; } Ok (result) }
};
}
