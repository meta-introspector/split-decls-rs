// Generated macro for scan_listitem (function)
macro_rules! Depcrate_scannersscan_listitem {
() => {
// Module: crate::scanners
// Provides: {"scan_listitem"}
// Dependencies: {}
# [doc = " return number of bytes scanned, delimiter, start index, and indent"] pub (crate) fn scan_listitem (bytes : & [u8]) -> Option < (usize , u8 , usize , usize) > { let mut c = * bytes . first () ? ; let (w , start) = match c { b'-' | b'+' | b'*' => (1 , 0) , b'0' ..= b'9' => { let (length , start) = parse_decimal (bytes , 9) ; c = * bytes . get (length) ? ; if ! (c == b'.' || c == b')') { return None ; } (length + 1 , start) } _ => { return None ; } } ; let (mut postn , mut postindent) = calc_indent (& bytes [w ..] , 5) ; if postindent == 0 { scan_eol (& bytes [w ..]) ? ; postindent += 1 ; } else if postindent > 4 { postn = 1 ; postindent = 1 ; } if scan_blank_line (& bytes [w ..]) . is_some () { postn = 0 ; postindent = 1 ; } Some ((w + postn , c , start , w + postindent)) }
};
}
