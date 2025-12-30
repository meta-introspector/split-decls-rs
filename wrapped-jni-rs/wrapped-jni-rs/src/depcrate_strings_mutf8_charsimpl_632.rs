// Generated macro for impl_632 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_632 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_632"}
// Dependencies: {}
impl < 'local , StringRef > From < MUTF8Chars < 'local , StringRef > > for JNIString where StringRef : AsRef < JString < 'local > > + Reference , { fn from (other : MUTF8Chars < 'local , StringRef >) -> JNIString { let jni_str : & JNIStr = & other ; jni_str . to_owned () } }
};
}
