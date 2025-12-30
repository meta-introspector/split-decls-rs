// Generated macro for scan_uri (function)
macro_rules! Depcrate_scannersscan_uri {
() => {
// Module: crate::scanners
// Provides: {"scan_uri"}
// Dependencies: {}
# [doc = " Returns (next_byte_offset, uri)"] fn scan_uri (text : & str , start_ix : usize) -> Option < (usize , CowStr < '_ >) > { let bytes = & text . as_bytes () [start_ix ..] ; if bytes . is_empty () || ! is_ascii_alpha (bytes [0]) { return None ; } let mut i = 1 ; while i < bytes . len () { let c = bytes [i] ; i += 1 ; match c { c if is_ascii_alphanumeric (c) => () , b'.' | b'-' | b'+' => () , b':' => break , _ => return None , } } if ! (3 ..= 33) . contains (& i) { return None ; } while i < bytes . len () { match bytes [i] { b'>' => return Some ((start_ix + i + 1 , text [start_ix .. (start_ix + i)] . into ())) , b'\0' ..= b' ' | b'<' => return None , _ => () , } i += 1 ; } None }
};
}
