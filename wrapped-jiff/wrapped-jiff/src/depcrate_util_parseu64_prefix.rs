// Generated macro for u64_prefix (function)
macro_rules! Depcrate_util_parseu64_prefix {
() => {
// Module: crate::util::parse
// Provides: {"u64_prefix"}
// Dependencies: {}
# [doc = " Parsed an optional `u64` that is a prefix of `bytes`."] # [doc = ""] # [doc = " If no digits (`[0-9]`) were found at the beginning of `bytes`, then `None`"] # [doc = " is returned."] # [doc = ""] # [doc = " Note that this is safe to call on untrusted input. It will not attempt"] # [doc = " to consume more input than could possibly fit into a parsed integer."] # [doc = ""] # [doc = " Since this returns a `u64`, it is possible that an integer that cannot"] # [doc = " fit into an `i64` is returned. Callers should handle this. (Indeed,"] # [doc = " `DurationUnits` handles this case.)"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When the parsed integer cannot fit into a `u64`."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn u64_prefix (bytes : & [u8]) -> Result < (Option < u64 > , & [u8]) , Error > { const MAX_U64_DIGITS : usize = 20 ; let mut digit_count = 0 ; let mut n : u64 = 0 ; while digit_count <= MAX_U64_DIGITS { let Some (& byte) = bytes . get (digit_count) else { break } ; if ! byte . is_ascii_digit () { break ; } digit_count += 1 ; let digit = u64 :: from (byte - b'0') ; n = n . checked_mul (10) . and_then (| n | n . checked_add (digit)) . ok_or_else (# [inline (never)] | | { err ! ("number `{}` too big to parse into 64-bit integer" , Bytes (& bytes [.. digit_count]) ,) } ,) ? ; } if digit_count == 0 { return Ok ((None , bytes)) ; } Ok ((Some (n) , & bytes [digit_count ..])) }
};
}
