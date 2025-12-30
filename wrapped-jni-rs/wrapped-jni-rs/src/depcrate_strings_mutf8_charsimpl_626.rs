// Generated macro for impl_626 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_626 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_626"}
// Dependencies: {}
impl < 'local , StringRef > std :: fmt :: Debug for MUTF8Chars < 'local , StringRef > where StringRef : AsRef < JString < 'local > > + Reference , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MUTF8Chars") . field ("obj" , self . obj . as_ref ()) . field ("chars" , & self . chars) . field ("is_copy" , & self . is_copy) . finish () } }
};
}
