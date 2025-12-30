// Generated macro for single (function)
macro_rules! Depcrate_singlesingle {
() => {
// Module: crate::single
// Provides: {"single"}
// Dependencies: {}
# [doc = " Transforms the given `value` to be suitable for use as an argument for Bourne shells by wrapping it into single quotes."] # [doc = ""] # [doc = " Every single-quote `'` is escaped as `'\\''`, every exclamation mark `!` is escaped as `'\\!'`,"] # [doc = " and the entire string is enclosed in single quotes."] pub fn single (mut value : & BStr) -> BString { let mut quoted = BString :: new (b"'" . to_vec ()) ; while let Some (pos) = value . find_byteset (b"'!") { quoted . extend_from_slice (& value [.. pos]) ; quoted . push_str (br"'\") ; quoted . push (value [pos]) ; quoted . push (b'\'') ; value = & value [pos + 1 ..] ; } quoted . extend_from_slice (value) ; quoted . push (b'\'') ; quoted }
};
}
