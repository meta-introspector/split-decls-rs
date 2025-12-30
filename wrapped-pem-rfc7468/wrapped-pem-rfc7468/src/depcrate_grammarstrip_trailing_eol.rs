// Generated macro for strip_trailing_eol (function)
macro_rules! Depcrate_grammarstrip_trailing_eol {
() => {
// Module: crate::grammar
// Provides: {"strip_trailing_eol"}
// Dependencies: {}
# [doc = " Strip a newline (`eol`) from the end of the provided byte slice."] # [doc = ""] # [doc = " The newline is considered mandatory and a decoding error will occur if it"] # [doc = " is not present."] # [doc = ""] # [doc = " From RFC 7468 Section 3:"] # [doc = " > lines are divided with CRLF, CR, or LF."] pub (crate) fn strip_trailing_eol (bytes : & [u8]) -> Option < & [u8] > { match bytes { [head @ .. , CHAR_CR , CHAR_LF] => Some (head) , [head @ .. , CHAR_LF] => Some (head) , [head @ .. , CHAR_CR] => Some (head) , _ => None , } }
};
}
