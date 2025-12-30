// Generated macro for scan_metadata_block (function)
macro_rules! Depcrate_scannersscan_metadata_block {
() => {
// Module: crate::scanners
// Provides: {"scan_metadata_block"}
// Dependencies: {}
# [doc = " Scan metadata block, returning the number of delimiter bytes"] # [doc = " (always 3 for now) and the delimiter character."] # [doc = ""] # [doc = " Differently to code blocks, metadata blocks must be closed with the closing"] # [doc = " sequence not being a valid terminator the end of the file."] # [doc = ""] # [doc = " In addition, they cannot be empty (closing sequence in the next line) and"] # [doc = " the next line cannot be an empty line."] pub (crate) fn scan_metadata_block (data : & [u8] , yaml_style_enabled : bool , pluses_style_enabled : bool ,) -> Option < (usize , u8) > { if yaml_style_enabled || pluses_style_enabled { let c = * data . first () ? ; if ! ((c == b'-' && yaml_style_enabled) || (c == b'+' && pluses_style_enabled)) { return None ; } let i = 1 + scan_ch_repeat (& data [1 ..] , c) ; let next_line = scan_nextline (& data [i ..]) ; for c in & data [i .. i + next_line] { if ! c . is_ascii_whitespace () { return None ; } } if i == 3 { let mut j = i ; let mut first_line = true ; while j < data . len () { j += scan_nextline (& data [j ..]) ; let closed = scan_closing_metadata_block (& data [j ..] , c) . is_some () ; if first_line { if closed || scan_blank_line (& data [j ..]) . is_some () { return None ; } first_line = false ; } if closed { return Some ((i , c)) ; } } None } else { None } } else { None } }
};
}
