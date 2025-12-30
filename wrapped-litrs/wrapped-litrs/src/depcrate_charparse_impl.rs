// Generated macro for parse_impl (function)
macro_rules! Depcrate_charparse_impl {
() => {
// Module: crate::char
// Provides: {"parse_impl"}
// Dependencies: {}
# [doc = " Precondition: first character in input must be `'`."] # [inline (never)] pub (crate) fn parse_impl (input : & str) -> Result < (char , usize) , ParseError > { let first = input . chars () . nth (1) . ok_or (perr (None , UnterminatedCharLiteral)) ? ; let (c , len) = match first { '\'' if input . chars () . nth (2) == Some ('\'') => return Err (perr (1 , UnescapedSingleQuote)) , '\'' => return Err (perr (None , EmptyCharLiteral)) , '\n' | '\t' | '\r' => return Err (perr (1 , UnescapedSpecialWhitespace)) , '\\' => { let (v , len) = unescape (& input [1 ..] , true , false , true) . map_err (| e | e . offset_span (1)) ? ; (v . unwrap_char () , len) } other => (other , other . len_utf8 ()) , } ; match input [1 + len ..] . find ('\'') { Some (0) => { } Some (_) => return Err (perr (None , OverlongCharLiteral)) , None => return Err (perr (None , UnterminatedCharLiteral)) , } let start_suffix = 1 + len + 1 ; let suffix = & input [start_suffix ..] ; check_suffix (suffix) . map_err (| kind | perr (start_suffix , kind)) ? ; Ok ((c , start_suffix)) }
};
}
