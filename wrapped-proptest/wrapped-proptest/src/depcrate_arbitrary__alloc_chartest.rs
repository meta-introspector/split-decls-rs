// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__alloc_chartest {
() => {
// Module: crate::arbitrary::_alloc::char
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (escape_debug => EscapeDebug , escape_default => EscapeDefault , escape_unicode => EscapeUnicode , parse_char_error => ParseCharError , decode_utf16_error => DecodeUtf16Error) ; no_panic_test ! (decode_utf16 => DecodeUtf16 << Vec < u16 > as IntoIterator >:: IntoIter >) ; # [cfg (feature = "unstable")] no_panic_test ! (to_lowercase => ToLowercase , to_uppercase => ToUppercase , char_try_from_error => CharTryFromError) ; }
};
}
