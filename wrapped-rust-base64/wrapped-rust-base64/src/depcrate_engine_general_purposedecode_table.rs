// Generated macro for decode_table (function)
macro_rules! Depcrate_engine_general_purposedecode_table {
() => {
// Module: crate::engine::general_purpose
// Provides: {"decode_table"}
// Dependencies: {}
# [doc = " Returns a table mapping base64 bytes as the lookup index to either:"] # [doc = " - [`INVALID_VALUE`] for bytes that aren't members of the alphabet"] # [doc = " - a byte whose lower 6 bits are the value that was encoded into the index byte"] pub (crate) const fn decode_table (alphabet : & Alphabet) -> [u8 ; 256] { let mut decode_table = [INVALID_VALUE ; 256] ; let mut index = 0 ; while index < 64 { decode_table [alphabet . symbols [index] as usize] = index as u8 ; index += 1 ; } decode_table }
};
}
