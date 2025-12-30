// Generated macro for impl_31 (impl)
macro_rules! Depcrate_eol_utilsimpl_31 {
() => {
// Module: crate::eol::utils
// Provides: {"impl_31"}
// Dependencies: {}
impl Stats { # [doc = " Gather statistics from the given `bytes`."] # [doc = ""] # [doc = " Note that the entire buffer will be scanned."] pub fn from_bytes (bytes : & [u8]) -> Self { let mut bytes = bytes . iter () . peekable () ; let mut null = 0 ; let mut lone_cr = 0 ; let mut lone_lf = 0 ; let mut crlf = 0 ; let mut printable = 0 ; let mut non_printable = 0 ; while let Some (b) = bytes . next () { if * b == b'\r' { match bytes . peek () { Some (n) if * * n == b'\n' => { bytes . next () ; crlf += 1 ; } _ => lone_cr += 1 , } continue ; } if * b == b'\n' { lone_lf += 1 ; continue ; } if * b == 127 { non_printable += 1 ; } else if * b < 32 { match * b { 8 | b'\t' | 27 | 12 => printable += 1 , 0 => { non_printable += 1 ; null += 1 ; } , _ => non_printable += 1 , } } else { printable += 1 ; } } Self { null , lone_cr , lone_lf , crlf , printable , non_printable , } } # [doc = " Returns `true` if these statistics are typical for a binary file."] pub fn is_binary (& self) -> bool { self . lone_cr > 0 || self . null > 0 || (self . printable >> 7) < self . non_printable } # [doc = " Return `true` if we would convert the buffer from which these stats are derived, knowing only the digest"] pub fn will_convert_lf_to_crlf (& self , digest : AttributesDigest , config : Configuration) -> bool { if digest . to_eol (config) != Some (Mode :: CrLf) { return false ; } if self . lone_lf == 0 { return false ; } if digest . is_auto_text () { if self . is_binary () { return false ; } if self . lone_cr > 0 || self . crlf > 0 { return false ; } } true } }
};
}
