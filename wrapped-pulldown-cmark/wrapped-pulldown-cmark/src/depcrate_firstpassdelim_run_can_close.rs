// Generated macro for delim_run_can_close (function)
macro_rules! Depcrate_firstpassdelim_run_can_close {
() => {
// Module: crate::firstpass
// Provides: {"delim_run_can_close"}
// Dependencies: {}
# [doc = " Determines whether the delimiter run starting at given index is"] # [doc = " right-flanking, as defined by the commonmark spec (and isn't intraword"] # [doc = " for _ delims)"] fn delim_run_can_close (s : & str , suffix : & str , run_len : usize , ix : usize , mode : TableParseMode , options : Options ,) -> bool { if ix == 0 { return false ; } let prev_char = s [.. ix] . chars () . last () . unwrap () ; if prev_char . is_whitespace () { return false ; } let next_char = if let Some (c) = suffix [run_len ..] . chars () . next () { c } else { return true ; } ; if mode == TableParseMode :: Active { if s . as_bytes () [.. ix] . ends_with (b"|") && ! s . as_bytes () [.. ix] . ends_with (br"\|") { return false ; } if next_char == '|' { return true ; } } let delim = suffix . bytes () . next () . unwrap () ; if (delim == b'*' || delim == b'^' || (delim == b'~' && run_len > 1)) && ! is_punctuation (prev_char) { return true ; } if delim == b'~' && (prev_char == '~' || options . contains (Options :: ENABLE_SUBSCRIPT)) { return true ; } next_char . is_whitespace () || is_punctuation (next_char) }
};
}
