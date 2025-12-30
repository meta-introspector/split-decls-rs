// Generated macro for is_valid_mutf8_cstr (function)
macro_rules! Depcrate_strings_ffi_stris_valid_mutf8_cstr {
() => {
// Module: crate::strings::ffi_str
// Provides: {"is_valid_mutf8_cstr"}
// Dependencies: {}
# [doc = " Returns true iff the given `CStr` has a valid *modified UTF-8* encoding."] # [doc = " Rules enforced:"] # [doc = " - ASCII 0x01..0x7F allowed (0x00 cannot appear inside `CStr::to_bytes()`)."] # [doc = " - U+0000 must be encoded as 0xC0 0x80 (accepted)."] # [doc = " - 2-byte: lead 0xC2..0xDF with one continuation."] # [doc = " - 3-byte: lead 0xE0..0xEF with two continuations; special overlong guard for 0xE0 (b1>=0xA0)."] # [doc = " - Surrogate range (0xED 0xA0..0xBF 0x80..0xBF) is **allowed** (that's how MUTF-8 represents supplementary chars)."] # [doc = " - 4-byte leads (0xF0..0xF7) and beyond are **rejected**."] const fn is_valid_mutf8_cstr (cstr : & CStr) -> bool { let bytes = cstr . to_bytes () ; let mut i = 0 ; while i < bytes . len () { let b0 = bytes [i] ; if b0 < 0x80 { i += 1 ; continue ; } if b0 == 0xC0 { if i + 1 >= bytes . len () { return false ; } let b1 = bytes [i + 1] ; if b1 != 0x80 { return false ; } i += 2 ; continue ; } if b0 >= 0xC2 && b0 <= 0xDF { if i + 1 >= bytes . len () { return false ; } let b1 = bytes [i + 1] ; if (b1 & 0xC0) != 0x80 { return false ; } i += 2 ; continue ; } if b0 >= 0xE0 && b0 <= 0xEF { if i + 2 >= bytes . len () { return false ; } let b1 = bytes [i + 1] ; let b2 = bytes [i + 2] ; if b0 == 0xE0 { if ! (b1 >= 0xA0 && b1 <= 0xBF) || (b2 & 0xC0) != 0x80 { return false ; } } else { if (b1 & 0xC0) != 0x80 || (b2 & 0xC0) != 0x80 { return false ; } } i += 3 ; continue ; } return false ; } true }
};
}
