// Generated macro for decode_latin1 (function)
macro_rules! Depcrate_memdecode_latin1 {
() => {
// Module: crate::mem
// Provides: {"decode_latin1"}
// Dependencies: {}
# [doc = " Converts bytes whose unsigned value is interpreted as Unicode code point"] # [doc = " (i.e. U+0000 to U+00FF, inclusive) to UTF-8."] # [doc = ""] # [doc = " Borrows if input is ASCII-only. Performs a single heap allocation"] # [doc = " otherwise."] # [doc = ""] # [doc = " Only available if the `alloc` feature is enabled (enabled by default)."] # [cfg (feature = "alloc")] pub fn decode_latin1 < 'a > (bytes : & 'a [u8]) -> Cow < 'a , str > { let up_to = ascii_valid_up_to (bytes) ; if up_to >= bytes . len () { debug_assert_eq ! (up_to , bytes . len ()) ; let s : & str = unsafe { :: core :: str :: from_utf8_unchecked (bytes) } ; return Cow :: Borrowed (s) ; } let (head , tail) = bytes . split_at (up_to) ; let capacity = head . len () + tail . len () * 2 ; let mut vec = Vec :: with_capacity (capacity) ; unsafe { vec . set_len (capacity) ; } (& mut vec [.. up_to]) . copy_from_slice (head) ; let written = convert_latin1_to_utf8 (tail , & mut vec [up_to ..]) ; vec . truncate (up_to + written) ; Cow :: Owned (unsafe { String :: from_utf8_unchecked (vec) }) }
};
}
