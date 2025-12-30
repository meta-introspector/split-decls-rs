// Generated macro for i64 (function)
macro_rules! Depcrate_util_parsei64 {
() => {
// Module: crate::util::parse
// Provides: {"i64"}
// Dependencies: {}
# [doc = " Parses an `i64` number from the beginning to the end of the given slice of"] # [doc = " ASCII digit characters."] # [doc = ""] # [doc = " If any byte in the given slice is not `[0-9]`, then this returns an error."] # [doc = " Similarly, if the number parsed does not fit into a `i64`, then this"] # [doc = " returns an error. Notably, this routine does not permit parsing a negative"] # [doc = " integer. (We use `i64` because everything in this crate uses signed"] # [doc = " integers, and because a higher level routine might want to parse the sign"] # [doc = " and then apply it to the result of this routine.)"] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn i64 (bytes : & [u8]) -> Result < i64 , Error > { if bytes . is_empty () { return Err (err ! ("invalid number, no digits found")) ; } let mut n : i64 = 0 ; for & byte in bytes { let digit = match byte . checked_sub (b'0') { None => { return Err (err ! ("invalid digit, expected 0-9 but got {}" , Byte (byte) ,)) ; } Some (digit) if digit > 9 => { return Err (err ! ("invalid digit, expected 0-9 but got {}" , Byte (byte) ,)) } Some (digit) => { debug_assert ! ((0 ..= 9) . contains (& digit)) ; i64 :: from (digit) } } ; n = n . checked_mul (10) . and_then (| n | n . checked_add (digit)) . ok_or_else (| | { err ! ("number '{}' too big to parse into 64-bit integer" , Bytes (bytes) ,) } ,) ? ; } Ok (n) }
};
}
