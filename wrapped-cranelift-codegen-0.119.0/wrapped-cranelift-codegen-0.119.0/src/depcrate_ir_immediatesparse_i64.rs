// Generated macro for parse_i64 (function)
macro_rules! Depcrate_ir_immediatesparse_i64 {
() => {
// Module: crate::ir::immediates
// Provides: {"parse_i64"}
// Dependencies: {}
# [doc = " Parse a 64-bit signed number."] fn parse_i64 (s : & str) -> Result < i64 , & 'static str > { let negative = s . starts_with ('-') ; let s2 = if negative || s . starts_with ('+') { & s [1 ..] } else { s } ; let mut value = parse_u64 (s2) ? ; if negative { value = value . wrapping_neg () ; if value as i64 > 0 { return Err ("Negative number too small") ; } } Ok (value as i64) }
};
}
