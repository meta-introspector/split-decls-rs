// Generated macro for encode_table (function)
macro_rules! Depcrate_engine_general_purposeencode_table {
() => {
// Module: crate::engine::general_purpose
// Provides: {"encode_table"}
// Dependencies: {}
# [doc = " Returns a table mapping a 6-bit index to the ASCII byte encoding of the index"] pub (crate) const fn encode_table (alphabet : & Alphabet) -> [u8 ; 64] { let mut encode_table = [0_u8 ; 64] ; { let mut index = 0 ; while index < 64 { encode_table [index] = alphabet . symbols [index] ; index += 1 ; } } encode_table }
};
}
