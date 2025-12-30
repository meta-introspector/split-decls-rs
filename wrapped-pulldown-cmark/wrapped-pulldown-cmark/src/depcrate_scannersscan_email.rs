// Generated macro for scan_email (function)
macro_rules! Depcrate_scannersscan_email {
() => {
// Module: crate::scanners
// Provides: {"scan_email"}
// Dependencies: {}
# [doc = " Returns (next_byte_offset, email)"] fn scan_email (text : & str , start_ix : usize) -> Option < (usize , CowStr < '_ >) > { let bytes = & text . as_bytes () [start_ix ..] ; let mut i = 0 ; while i < bytes . len () { let c = bytes [i] ; i += 1 ; match c { c if is_ascii_alphanumeric (c) => () , b'.' | b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'/' | b'=' | b'?' | b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~' | b'-' => () , b'@' if i > 1 => break , _ => return None , } } loop { let label_start_ix = i ; let mut fresh_label = true ; while i < bytes . len () { match bytes [i] { c if is_ascii_alphanumeric (c) => () , b'-' if fresh_label => { return None ; } b'-' => () , _ => break , } fresh_label = false ; i += 1 ; } if i == label_start_ix || i - label_start_ix > 63 || bytes [i - 1] == b'-' { return None ; } if bytes . get (i) != Some (& b'.') { break ; } i += 1 ; } if bytes . get (i) != Some (& b'>') { return None ; } Some ((start_ix + i + 1 , text [start_ix .. (start_ix + i)] . into ())) }
};
}
