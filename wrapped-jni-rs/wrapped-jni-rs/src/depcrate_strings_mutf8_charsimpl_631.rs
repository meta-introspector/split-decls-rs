// Generated macro for impl_631 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_631 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_631"}
// Dependencies: {}
impl < 'local , 'java_str , StringRef > From < & 'java_str MUTF8Chars < 'local , StringRef > > for & 'java_str JNIStr where StringRef : AsRef < JString < 'local > > + Reference , { fn from (other : & 'java_str MUTF8Chars < 'local , StringRef >) -> & 'java_str JNIStr { unsafe { JNIStr :: from_ptr (other . chars) } } }
};
}
