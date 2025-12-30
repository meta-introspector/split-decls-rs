// Generated macro for from_bytes (function)
macro_rules! Depcrate_parse_nomfrom_bytes {
() => {
// Module: crate::parse::nom
// Provides: {"from_bytes"}
// Dependencies: {}
# [doc = " Attempt to zero-copy parse the provided bytes, passing results to `dispatch`."] pub fn from_bytes < 'i > (mut input : & 'i [u8] , dispatch : & mut dyn FnMut (Event < 'i >)) -> Result < () , Error > { let start = input . checkpoint () ; let bom = unicode_bom :: Bom :: from (input) ; input . next_slice (bom . len ()) ; repeat (0 .. , alt ((comment . map (Event :: Comment) , take_spaces1 . map (| whitespace | Event :: Whitespace (Cow :: Borrowed (whitespace))) , | i : & mut & 'i [u8] | { let newline = take_newlines1 . parse_next (i) ? ; let o = Event :: Newline (Cow :: Borrowed (newline)) ; Ok (o) } ,)) ,) . fold (| | () , | _acc , event | dispatch (event)) . parse_next (& mut input) . expect ("many0(alt(...)) panicked. Likely a bug in one of the children parsers.") ; if input . is_empty () { return Ok (()) ; } let mut node = ParseNode :: SectionHeader ; let res = repeat (1 .. , | i : & mut & 'i [u8] | section (i , & mut node , dispatch)) . map (| () | ()) . parse_next (& mut input) ; res . map_err (| _ | { let newlines = newlines_from (input , start) ; Error { line_number : newlines , last_attempted_parser : node , parsed_until : input . as_bstr () . into () , } }) ? ; if ! input . is_empty () { let newlines = newlines_from (input , start) ; return Err (Error { line_number : newlines , last_attempted_parser : node , parsed_until : input . as_bstr () . into () , }) ; } Ok (()) }
};
}
