// Generated macro for parse_ordinal (function)
macro_rules! Depcrate_read_pe_exportparse_ordinal {
() => {
// Module: crate::read::pe::export
// Provides: {"parse_ordinal"}
// Dependencies: {}
fn parse_ordinal (digits : & [u8]) -> Option < u32 > { if digits . is_empty () { return None ; } let mut result : u32 = 0 ; for & c in digits { let x = (c as char) . to_digit (10) ? ; result = result . checked_mul (10) ? . checked_add (x) ? ; } Some (result) }
};
}
