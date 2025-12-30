// Generated macro for impl_633 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_633 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_633"}
// Dependencies: {}
impl < 'local , 'java_str , StringRef > From < & 'java_str MUTF8Chars < 'local , StringRef > > for Cow < 'java_str , str > where StringRef : AsRef < JString < 'local > > + Reference , { fn from (other : & 'java_str MUTF8Chars < 'local , StringRef >) -> Cow < 'java_str , str > { let jni_str : & JNIStr = other ; jni_str . into () } }
};
}
