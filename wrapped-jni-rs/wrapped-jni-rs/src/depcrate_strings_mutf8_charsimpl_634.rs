// Generated macro for impl_634 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_634 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_634"}
// Dependencies: {}
impl < 'local , StringRef > From < MUTF8Chars < 'local , StringRef > > for String where StringRef : AsRef < JString < 'local > > + Reference , { fn from (other : MUTF8Chars < 'local , StringRef >) -> String { let cow : Cow < str > = (& other) . into () ; cow . into_owned () } }
};
}
