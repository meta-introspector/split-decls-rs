// Generated macro for strip_leading_eol (function)
macro_rules! Depcrate_grammarstrip_leading_eol {
() => {
// Module: crate::grammar
// Provides: {"strip_leading_eol"}
// Dependencies: {}
# [doc = " Strip a newline (`eol`) from the beginning of the provided byte slice."] # [doc = ""] # [doc = " The newline is considered mandatory and a decoding error will occur if it"] # [doc = " is not present."] # [doc = ""] # [doc = " From RFC 7468 Section 3:"] # [doc = " > lines are divided with CRLF, CR, or LF."] pub (crate) fn strip_leading_eol (bytes : & [u8]) -> Option < & [u8] > { match bytes { [CHAR_LF , rest @ ..] => Some (rest) , [CHAR_CR , CHAR_LF , rest @ ..] => Some (rest) , [CHAR_CR , rest @ ..] => Some (rest) , _ => None , } }
};
}
