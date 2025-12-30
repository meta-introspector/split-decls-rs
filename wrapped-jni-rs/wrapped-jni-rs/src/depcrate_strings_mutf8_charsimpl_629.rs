// Generated macro for impl_629 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_629 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_629"}
// Dependencies: {}
impl < 'local , StringRef > std :: fmt :: Display for MUTF8Chars < 'local , StringRef > where StringRef : AsRef < JString < 'local > > + Reference , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let jni_str : & JNIStr = self ; jni_str . fmt (f) } }
};
}
