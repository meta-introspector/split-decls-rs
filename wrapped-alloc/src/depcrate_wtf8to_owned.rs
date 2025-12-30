// Generated macro for to_owned (function)
macro_rules! Depcrate_wtf8to_owned {
() => {
// Module: crate::wtf8
// Provides: {"to_owned"}
// Dependencies: {}
# [doc = " Creates an owned `Wtf8Buf` from a borrowed `Wtf8`."] pub (super) fn to_owned (slice : & Wtf8) -> Wtf8Buf { Wtf8Buf { bytes : slice . as_bytes () . to_vec () , is_known_utf8 : false } }
};
}
