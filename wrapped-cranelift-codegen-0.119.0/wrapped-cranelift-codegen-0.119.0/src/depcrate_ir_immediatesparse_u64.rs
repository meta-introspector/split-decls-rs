// Generated macro for parse_u64 (function)
macro_rules! Depcrate_ir_immediatesparse_u64 {
() => {
// Module: crate::ir::immediates
// Provides: {"parse_u64"}
// Dependencies: {}
# [doc = " Parse a 64-bit unsigned number."] fn parse_u64 (s : & str) -> Result < u64 , & 'static str > { let mut value : u64 = 0 ; let mut digits = 0 ; if s . starts_with ("-0x") { return Err ("Invalid character in hexadecimal number") ; } else if let Some (num) = s . strip_prefix ("0x") { for ch in num . chars () { match ch . to_digit (16) { Some (digit) => { digits += 1 ; if digits > 16 { return Err ("Too many hexadecimal digits") ; } value = (value << 4) | u64 :: from (digit) ; } None => { if ch != '_' { return Err ("Invalid character in hexadecimal number") ; } } } } } else { for ch in s . chars () { match ch . to_digit (10) { Some (digit) => { digits += 1 ; match value . checked_mul (10) { None => return Err ("Too large decimal number") , Some (v) => value = v , } match value . checked_add (u64 :: from (digit)) { None => return Err ("Too large decimal number") , Some (v) => value = v , } } None => { if ch != '_' { return Err ("Invalid character in decimal number") ; } } } } } if digits == 0 { return Err ("No digits in number") ; } Ok (value) }
};
}
