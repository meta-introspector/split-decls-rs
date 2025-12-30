// Generated macro for bytes_to_escaped_literal (function)
macro_rules! Depcrate_globbytes_to_escaped_literal {
() => {
// Module: crate::glob
// Provides: {"bytes_to_escaped_literal"}
// Dependencies: {}
# [doc = " Converts an arbitrary sequence of bytes to a UTF-8 string. All non-ASCII"] # [doc = " code units are converted to their escaped form."] fn bytes_to_escaped_literal (bs : & [u8]) -> String { let mut s = String :: with_capacity (bs . len ()) ; for & b in bs { if b <= 0x7F { regex_syntax :: escape_into (char :: from (b) . encode_utf8 (& mut [0 ; 4]) , & mut s ,) ; } else { write ! (& mut s , "\\x{:02x}" , b) . unwrap () ; } } s }
};
}
