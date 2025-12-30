// Generated macro for parse_u64_digits (function)
macro_rules! Depcrate_read_archiveparse_u64_digits {
() => {
// Module: crate::read::archive
// Provides: {"parse_u64_digits"}
// Dependencies: {}
fn parse_u64_digits (digits : & [u8] , radix : u32) -> Option < u64 > { if let [b' ' , ..] = digits { return None ; } let mut result : u64 = 0 ; for & c in digits { if c == b' ' { return Some (result) ; } else { let x = (c as char) . to_digit (radix) ? ; result = result . checked_mul (u64 :: from (radix)) ? . checked_add (u64 :: from (x)) ? ; } } Some (result) }
};
}
