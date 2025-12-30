// Generated macro for delim_run_can_open (function)
macro_rules! Depcrate_firstpassdelim_run_can_open {
() => {
// Module: crate::firstpass
// Provides: {"delim_run_can_open"}
// Dependencies: {}
# [doc = " Determines whether the delimiter run starting at given index is"] # [doc = " left-flanking, as defined by the commonmark spec (and isn't intraword"] # [doc = " for _ delims)."] # [doc = " suffix is &s[ix..], which is passed in as an optimization, since taking"] # [doc = " a string subslice is O(n)."] fn delim_run_can_open (s : & str , suffix : & str , run_len : usize , ix : usize , mode : TableParseMode , options : Options ,) -> bool { let next_char = if let Some (c) = suffix [run_len ..] . chars () . next () { c } else { return false ; } ; if next_char . is_whitespace () { return false ; } if ix == 0 { return true ; } if mode == TableParseMode :: Active { if s . as_bytes () [.. ix] . ends_with (b"|") && ! s . as_bytes () [.. ix] . ends_with (br"\|") { return true ; } if next_char == '|' { return false ; } } let delim = suffix . bytes () . next () . unwrap () ; if (delim == b'*' || delim == b'^') && ! is_punctuation (next_char) { return true ; } if delim == b'~' && run_len > 1 { return true ; } let prev_char = s [.. ix] . chars () . last () . unwrap () ; if delim == b'~' && (prev_char == '~' || options . contains (Options :: ENABLE_SUBSCRIPT)) && ! is_punctuation (next_char) { return true ; } prev_char . is_whitespace () || is_punctuation (prev_char) && (delim != b'\'' || ! [']' , ')'] . contains (& prev_char)) }
};
}
