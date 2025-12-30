// Generated macro for to_cesu8_internal (function)
macro_rules! Depcrateto_cesu8_internal {
() => {
// Module: crate
// Provides: {"to_cesu8_internal"}
// Dependencies: {}
fn to_cesu8_internal (text : & str , variant : Variant) -> Vec < u8 > { let bytes = text . as_bytes () ; let mut encoded = Vec :: with_capacity (bytes . len () + bytes . len () >> 2) ; let mut i = 0 ; while i < bytes . len () { let b = bytes [i] ; if variant == Variant :: Java && b == 0 { encoded . push (0xc0) ; encoded . push (0x80) ; i += 1 ; } else if b < 128 { encoded . push (b) ; i += 1 ; } else { let w = utf8_char_width (b) ; assert ! (w <= 4) ; assert ! (i + w <= bytes . len ()) ; if w != 4 { encoded . extend (bytes [i .. i + w] . iter () . cloned ()) ; } else { let s = unsafe { from_utf8_unchecked (& bytes [i .. i + w]) } ; let c = s . chars () . next () . unwrap () as u32 - 0x10000 ; let mut s : [u16 ; 2] = [0 ; 2] ; s [0] = ((c >> 10) as u16) | 0xD800 ; s [1] = ((c & 0x3FF) as u16) | 0xDC00 ; encoded . extend (enc_surrogate (s [0]) . iter () . cloned ()) ; encoded . extend (enc_surrogate (s [1]) . iter () . cloned ()) ; } i += w ; } } encoded }
};
}
