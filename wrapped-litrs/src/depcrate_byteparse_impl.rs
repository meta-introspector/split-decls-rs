// Generated macro for parse_impl (function)
macro_rules! Depcrate_byteparse_impl {
() => {
// Module: crate::byte
// Provides: {"parse_impl"}
// Dependencies: {}
# [doc = " Precondition: must start with `b'`."] # [inline (never)] pub (crate) fn parse_impl (input : & str) -> Result < (u8 , usize) , ParseError > { let input_bytes = input . as_bytes () ; let first = input_bytes . get (2) . ok_or (perr (None , UnterminatedByteLiteral)) ? ; let (c , len) = match first { b'\'' if input_bytes . get (3) == Some (& b'\'') => return Err (perr (2 , UnescapedSingleQuote)) , b'\'' => return Err (perr (None , EmptyByteLiteral)) , b'\n' | b'\t' | b'\r' => return Err (perr (2 , UnescapedSpecialWhitespace)) , b'\\' => { let (v , len) = unescape (& input [2 ..] , false , true , true) . map_err (| e | e . offset_span (2)) ? ; (v . unwrap_byte () , len) } other if other . is_ascii () => (* other , 1) , _ => return Err (perr (2 , NonAsciiInByteLiteral)) , } ; match input [2 + len ..] . find ('\'') { Some (0) => { } Some (_) => return Err (perr (None , OverlongByteLiteral)) , None => return Err (perr (None , UnterminatedByteLiteral)) , } let start_suffix = 2 + len + 1 ; let suffix = & input [start_suffix ..] ; check_suffix (suffix) . map_err (| kind | perr (start_suffix , kind)) ? ; Ok ((c , start_suffix)) }
};
}
