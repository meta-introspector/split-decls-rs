// Generated macro for impl_364 (impl)
macro_rules! Depcrate_subtagsimpl_364 {
() => {
// Module: crate::subtags
// Provides: {"impl_364"}
// Dependencies: {}
# [expect (clippy :: len_without_is_empty)] impl Subtag { # [allow (dead_code)] pub (crate) const fn valid_key (v : & [u8]) -> bool { 2 <= v . len () && v . len () <= 8 } # [doc = " Returns the length of `self`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::subtag;"] # [doc = " let s = subtag!(\"foo\");"] # [doc = " assert_eq!(s.len(), 3);"] # [doc = " ```"] pub fn len (& self) -> usize { self . 0 . len () } # [doc (hidden)] pub fn from_tinystr_unvalidated (input : tinystr :: TinyAsciiStr < 8 >) -> Self { Self (input) } # [doc (hidden)] pub fn as_tinystr (& self) -> tinystr :: TinyAsciiStr < 8 > { self . 0 } # [allow (dead_code)] pub (crate) fn to_ascii_lowercase (self) -> Self { Self (self . 0 . to_ascii_lowercase ()) } }
};
}
