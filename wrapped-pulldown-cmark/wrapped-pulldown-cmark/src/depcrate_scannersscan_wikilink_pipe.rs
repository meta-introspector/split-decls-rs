// Generated macro for scan_wikilink_pipe (function)
macro_rules! Depcrate_scannersscan_wikilink_pipe {
() => {
// Module: crate::scanners
// Provides: {"scan_wikilink_pipe"}
// Dependencies: {}
pub (crate) fn scan_wikilink_pipe (data : & str , start_ix : usize , len : usize) -> Option < (usize , & str) > { let bytes = data . as_bytes () ; let end_ix = core :: cmp :: min (start_ix + len , bytes . len ()) ; let mut i = start_ix ; while i < end_ix { if bytes [i] == b'|' { return Some ((i + 1 , & data [start_ix .. i])) ; } i += 1 ; } None }
};
}
