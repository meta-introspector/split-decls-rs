// Generated macro for scan_hrule (function)
macro_rules! Depcrate_scannersscan_hrule {
() => {
// Module: crate::scanners
// Provides: {"scan_hrule"}
// Dependencies: {}
# [doc = " Scan hrule opening sequence."] # [doc = ""] # [doc = " Returns Ok(x) when it finds an hrule, where x is the"] # [doc = " size of line containing the hrule, including the trailing newline."] # [doc = ""] # [doc = " Returns Err(x) when it does not find an hrule and x is"] # [doc = " the offset in data before no hrule can appear."] pub (crate) fn scan_hrule (bytes : & [u8]) -> Result < usize , usize > { if bytes . len () < 3 { return Err (0) ; } let c = bytes [0] ; if ! (c == b'*' || c == b'-' || c == b'_') { return Err (0) ; } let mut n = 0 ; let mut i = 0 ; while i < bytes . len () { match bytes [i] { b'\n' | b'\r' => { i += scan_eol (& bytes [i ..]) . unwrap_or (0) ; break ; } c2 if c2 == c => { n += 1 ; } b' ' | b'\t' => () , _ => return Err (i) , } i += 1 ; } if n >= 3 { Ok (i) } else { Err (i) } }
};
}
