// Generated macro for scan_raw_string (function)
macro_rules! Depcrate_escapescan_raw_string {
() => {
// Module: crate::escape
// Provides: {"scan_raw_string"}
// Dependencies: {}
# [doc = " Reads and checks a raw (byte) string literal. Returns the number of hashes"] # [doc = " and the index when the suffix starts."] # [inline (never)] pub (crate) fn scan_raw_string (input : & str , offset : usize , unicode : bool , allow_nul : bool ,) -> Result < (u8 , usize) , ParseError > { let num_hashes = input [offset ..] . bytes () . position (| b | b != b'#') . ok_or (perr (None , InvalidLiteral)) ? ; if num_hashes > 256 { return Err (perr (offset .. offset + num_hashes , TooManyHashes)) ; } if input . as_bytes () . get (offset + num_hashes) != Some (& b'"') { return Err (perr (None , InvalidLiteral)) ; } let start_inner = offset + num_hashes + 1 ; let hashes = & input [offset .. num_hashes + offset] ; let mut closing_quote_pos = None ; let mut i = start_inner ; while i < input . len () { let b = input . as_bytes () [i] ; if b == b'"' && input [i + 1 ..] . starts_with (hashes) { closing_quote_pos = Some (i) ; break ; } if b == b'\r' { return Err (perr (i , CarriageReturn)) ; } if b == b'\0' && ! allow_nul { return Err (perr (i , NulByte)) ; } if ! unicode { if ! b . is_ascii () { return Err (perr (i , NonAsciiInByteLiteral)) ; } } i += 1 ; } let closing_quote_pos = closing_quote_pos . ok_or (perr (None , UnterminatedRawString)) ? ; let start_suffix = closing_quote_pos + num_hashes + 1 ; let suffix = & input [start_suffix ..] ; check_suffix (suffix) . map_err (| kind | perr (start_suffix , kind)) ? ; Ok ((num_hashes as u8 , start_suffix)) }
};
}
