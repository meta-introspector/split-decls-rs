// Generated macro for impl_630 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_630 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_630"}
// Dependencies: {}
impl < 'local , StringRef > :: std :: ops :: Deref for MUTF8Chars < 'local , StringRef > where StringRef : AsRef < JString < 'local > > + Reference , { type Target = JNIStr ; fn deref (& self) -> & Self :: Target { self . into () } }
};
}
