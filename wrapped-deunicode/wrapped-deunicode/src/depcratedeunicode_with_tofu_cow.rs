// Generated macro for deunicode_with_tofu_cow (function)
macro_rules! Depcratedeunicode_with_tofu_cow {
() => {
// Module: crate
// Provides: {"deunicode_with_tofu_cow"}
// Dependencies: {}
# [doc = " Same as [`deunicode_with_tofu()`], but avoids allocating a new `String` if not necessary."] # [doc = ""] # [doc = " You can use \"\\u{FFFD}\" to use the usual Unicode Replacement Character."] # [doc = ""] # [doc = " \"Tofu\" is a nickname for a replacement character, which in Unicode fonts usually"] # [doc = " looks like a block of tofu."] # [cfg (feature = "alloc")] # [must_use] pub fn deunicode_with_tofu_cow < 'input > (s : & 'input str , custom_placeholder : & str) -> Cow < 'input , str > { let ascii_len = s . as_bytes () . iter () . take_while (| & & c | c < 0x7F) . count () ; if ascii_len >= s . len () { return Cow :: Borrowed (s) ; } let (ascii , rest) = s . as_bytes () . split_at (ascii_len) ; debug_assert ! (core :: str :: from_utf8 (ascii) . is_ok ()) ; let ascii = unsafe { core :: str :: from_utf8_unchecked (ascii) } ; let mut out = String :: new () ; out . try_reserve_exact (s . len () | 15) . unwrap_or_else (| _ | panic ! ()) ; let needs_to_grow = ascii . as_bytes () . len () > out . capacity () . wrapping_sub (out . len ()) ; if ! needs_to_grow { out . push_str (ascii) ; } debug_assert ! (core :: str :: from_utf8 (rest) . is_ok ()) ; let s = unsafe { core :: str :: from_utf8_unchecked (rest) } ; out . extend (s . ascii_chars () . map (move | ch | ch . unwrap_or (custom_placeholder))) ; Cow :: Owned (out) }
};
}
