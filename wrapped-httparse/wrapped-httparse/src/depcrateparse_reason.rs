// Generated macro for parse_reason (function)
macro_rules! Depcrateparse_reason {
() => {
// Module: crate
// Provides: {"parse_reason"}
// Dependencies: {}
# [doc = " From [RFC 7230](https://tools.ietf.org/html/rfc7230):"] # [doc = ""] # [doc = " > ```notrust"] # [doc = " > reason-phrase  = *( HTAB / SP / VCHAR / obs-text )"] # [doc = " > HTAB           = %x09        ; horizontal tab"] # [doc = " > VCHAR          = %x21-7E     ; visible (printing) characters"] # [doc = " > obs-text       = %x80-FF"] # [doc = " > ```"] # [doc = ""] # [doc = " > A.2.  Changes from RFC 2616"] # [doc = " >"] # [doc = " > Non-US-ASCII content in header fields and the reason phrase"] # [doc = " > has been obsoleted and made opaque (the TEXT rule was removed)."] # [inline] fn parse_reason < 'a > (bytes : & mut Bytes < 'a >) -> Result < & 'a str > { let mut seen_obs_text = false ; loop { let b = next ! (bytes) ; if b == b'\r' { expect ! (bytes . next () == b'\n' => Err (Error :: Status)) ; return Ok (Status :: Complete (unsafe { let bytes = bytes . slice_skip (2) ; if ! seen_obs_text { str :: from_utf8_unchecked (bytes) } else { "" } } ,)) ; } else if b == b'\n' { return Ok (Status :: Complete (unsafe { let bytes = bytes . slice_skip (1) ; if ! seen_obs_text { str :: from_utf8_unchecked (bytes) } else { "" } } ,)) ; } else if ! (b == 0x09 || b == b' ' || (0x21 ..= 0x7E) . contains (& b) || b >= 0x80) { return Err (Error :: Status) ; } else if b >= 0x80 { seen_obs_text = true ; } } }
};
}
