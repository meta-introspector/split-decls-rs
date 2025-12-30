// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [doc = " Use `.map(|ch| ch.unwrap_or(\"?\"))` to replace invalid characters."] impl < 'a > AsciiCharsIter < 'a > { # [inline] pub fn new (unicode_string : & 'a str) -> Self { let mut chars = unicode_string . chars () ; Self { next_char : chars . next () . map (deunicode_char) , chars , } } }
};
}
