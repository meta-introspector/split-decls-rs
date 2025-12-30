// Generated macro for unescape_llvm_string_contents (function)
macro_rules! Depcrate_llvm_utilsunescape_llvm_string_contents {
() => {
// Module: crate::llvm_utils
// Provides: {"unescape_llvm_string_contents"}
// Dependencies: {}
# [doc = " Given the raw contents of a string literal in LLVM IR assembly, decodes any"] # [doc = " backslash escapes and returns a vector containing the resulting byte string."] pub (crate) fn unescape_llvm_string_contents (contents : & str) -> Vec < u8 > { let escape_re = { static RE : OnceLock < bytes :: Regex > = OnceLock :: new () ; RE . get_or_init (| | bytes :: Regex :: new (r"\\\\|\\([0-9A-Za-z]{2})") . unwrap ()) } ; fn u8_from_hex_digits (digits : & [u8]) -> u8 { assert_eq ! (digits . len () , 2) ; let digits = std :: str :: from_utf8 (digits) . unwrap () ; u8 :: from_str_radix (digits , 16) . unwrap () } escape_re . replace_all (contents . as_bytes () , | captures : & bytes :: Captures < '_ > | { let byte = match captures . get (1) { None => b'\\' , Some (hex_digits) => u8_from_hex_digits (hex_digits . as_bytes ()) , } ; [byte] }) . into_owned () }
};
}
