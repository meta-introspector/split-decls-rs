// Generated macro for scan_link_dest (function)
macro_rules! Depcrate_scannersscan_link_dest {
() => {
// Module: crate::scanners
// Provides: {"scan_link_dest"}
// Dependencies: {}
pub (crate) fn scan_link_dest (data : & str , start_ix : usize , max_next : usize ,) -> Option < (usize , & str) > { let bytes = & data . as_bytes () [start_ix ..] ; let mut i = scan_ch (bytes , b'<') ; if i != 0 { while i < bytes . len () { match bytes [i] { b'\n' | b'\r' | b'<' => return None , b'>' => return Some ((i + 1 , & data [(start_ix + 1) .. (start_ix + i)])) , b'\\' if i + 1 < bytes . len () && is_ascii_punctuation (bytes [i + 1]) => { i += 1 ; } _ => { } } i += 1 ; } None } else { let mut nest = 0 ; while i < bytes . len () { match bytes [i] { 0x0 ..= 0x20 => { break ; } b'(' => { if nest > max_next { return None ; } nest += 1 ; } b')' => { if nest == 0 { break ; } nest -= 1 ; } b'\\' if i + 1 < bytes . len () && is_ascii_punctuation (bytes [i + 1]) => { i += 1 ; } _ => { } } i += 1 ; } if nest != 0 { return None ; } Some ((i , & data [start_ix .. (start_ix + i)])) } }
};
}
