// Generated macro for first_byte_or_empty (function)
macro_rules! Depcrate_parsefirst_byte_or_empty {
() => {
// Module: crate::parse
// Provides: {"first_byte_or_empty"}
// Dependencies: {}
pub (crate) fn first_byte_or_empty (s : & str) -> Result < u8 , ParseError > { s . as_bytes () . first () . copied () . ok_or (perr (None , Empty)) }
};
}
