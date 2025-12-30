// Generated macro for fraction (function)
macro_rules! Depcrate_util_parsefraction {
() => {
// Module: crate::util::parse
// Provides: {"fraction"}
// Dependencies: {}
# [doc = " Parses a `u32` fractional number from the beginning to the end of the given"] # [doc = " slice of ASCII digit characters."] # [doc = ""] # [doc = " The fraction's maximum precision is always 9 digits. The returned integer"] # [doc = " will always be in units of `10^{max_precision}`. For example, this"] # [doc = " will parse a fractional amount of seconds with a maximum precision of"] # [doc = " nanoseconds."] # [doc = ""] # [doc = " If any byte in the given slice is not `[0-9]`, then this returns an error."] # [doc = " Notably, this routine does not permit parsing a negative integer."] pub (crate) fn fraction (bytes : & [u8]) -> Result < u32 , Error > { const MAX_PRECISION : usize = 9 ; if bytes . is_empty () { return Err (err ! ("invalid fraction, no digits found")) ; } else if bytes . len () > MAX_PRECISION { return Err (err ! ("invalid fraction, too many digits \
             (at most {MAX_PRECISION} are allowed")) ; } let mut n : u32 = 0 ; for & byte in bytes { let digit = match byte . checked_sub (b'0') { None => { return Err (err ! ("invalid fractional digit, expected 0-9 but got {}" , Byte (byte) ,)) ; } Some (digit) if digit > 9 => { return Err (err ! ("invalid fractional digit, expected 0-9 but got {}" , Byte (byte) ,)) } Some (digit) => { debug_assert ! ((0 ..= 9) . contains (& digit)) ; u32 :: from (digit) } } ; n = n . checked_mul (10) . and_then (| n | n . checked_add (digit)) . ok_or_else (| | { err ! ("fractional '{}' too big to parse into 64-bit integer" , Bytes (bytes) ,) } ,) ? ; } for _ in bytes . len () .. MAX_PRECISION { n = n . checked_mul (10) . ok_or_else (| | { err ! ("fractional '{}' too big to parse into 64-bit integer \
                 (too much precision supported)" , Bytes (bytes)) }) ? ; } Ok (n) }
};
}
