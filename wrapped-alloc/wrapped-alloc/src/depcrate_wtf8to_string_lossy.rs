// Generated macro for to_string_lossy (function)
macro_rules! Depcrate_wtf8to_string_lossy {
() => {
// Module: crate::wtf8
// Provides: {"to_string_lossy"}
// Dependencies: {}
# [doc = " Lossily converts the string to UTF-8."] # [doc = " Returns a UTF-8 `&str` slice if the contents are well-formed in UTF-8."] # [doc = ""] # [doc = " Surrogates are replaced with `\"\\u{FFFD}\"` (the replacement character “�”)."] # [doc = ""] # [doc = " This only copies the data if necessary (if it contains any surrogate)."] pub (super) fn to_string_lossy (slice : & Wtf8) -> Cow < '_ , str > { let Some ((surrogate_pos , _)) = slice . next_surrogate (0) else { return Cow :: Borrowed (unsafe { str :: from_utf8_unchecked (slice . as_bytes ()) }) ; } ; let wtf8_bytes = slice . as_bytes () ; let mut utf8_bytes = Vec :: with_capacity (slice . len ()) ; utf8_bytes . extend_from_slice (& wtf8_bytes [.. surrogate_pos]) ; utf8_bytes . extend_from_slice ("\u{FFFD}" . as_bytes ()) ; let mut pos = surrogate_pos + 3 ; loop { match slice . next_surrogate (pos) { Some ((surrogate_pos , _)) => { utf8_bytes . extend_from_slice (& wtf8_bytes [pos .. surrogate_pos]) ; utf8_bytes . extend_from_slice ("\u{FFFD}" . as_bytes ()) ; pos = surrogate_pos + 3 ; } None => { utf8_bytes . extend_from_slice (& wtf8_bytes [pos ..]) ; return Cow :: Owned (unsafe { String :: from_utf8_unchecked (utf8_bytes) }) ; } } } }
};
}
