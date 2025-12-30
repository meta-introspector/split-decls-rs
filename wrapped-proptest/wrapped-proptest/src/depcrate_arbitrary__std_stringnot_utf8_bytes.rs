// Generated macro for not_utf8_bytes (function)
macro_rules! Depcrate_arbitrary__std_stringnot_utf8_bytes {
() => {
// Module: crate::arbitrary::_std::string
// Provides: {"not_utf8_bytes"}
// Dependencies: {}
# [doc = " This strategy produces sequences of bytes that are guaranteed to be illegal"] # [doc = " wrt. UTF-8 with the goal of producing a suffix of bytes in the end of"] # [doc = " an otherwise legal UTF-8 string that causes the string to be illegal."] # [doc = " This is used primarily to generate the `Utf8Error` type and similar."] pub (crate) fn not_utf8_bytes (allow_null : bool ,) -> impl Strategy < Value = Vec < u8 > > { let prefix = collection :: vec (any :: < char > () , .. :: std :: u16 :: MAX as usize) ; let suffix = gen_el_bytes (allow_null) ; (prefix , suffix) . prop_map (move | (prefix_bytes , el_bytes) | { let iter = prefix_bytes . iter () ; let string : String = if allow_null { iter . collect () } else { iter . filter (| & & x | x != '\u{0}') . collect () } ; let mut bytes = string . into_bytes () ; bytes . extend (el_bytes . into_iter ()) ; bytes }) }
};
}
