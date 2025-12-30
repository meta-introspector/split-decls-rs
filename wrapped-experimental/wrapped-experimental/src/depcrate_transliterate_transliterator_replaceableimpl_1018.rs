// Generated macro for impl_1018 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1018 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1018"}
// Dependencies: {}
impl TransliteratorBuffer { pub (crate) fn from_string (s : String) -> Self { Self (s . into_bytes ()) } pub (crate) fn into_string (self) -> String { debug_assert ! (core :: str :: from_utf8 (& self . 0) . is_ok ()) ; unsafe { String :: from_utf8_unchecked (self . 0) } } }
};
}
