// Generated macro for parse_le_u16 (function)
macro_rules! Depcrate_gzparse_le_u16 {
() => {
// Module: crate::gz
// Provides: {"parse_le_u16"}
// Dependencies: {}
fn parse_le_u16 (buffer : & [u8 ; 2]) -> u16 { u16 :: from_le_bytes (* buffer) }
};
}
