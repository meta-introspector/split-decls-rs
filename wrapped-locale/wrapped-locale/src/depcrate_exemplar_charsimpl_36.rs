// Generated macro for impl_36 (impl)
macro_rules! Depcrate_exemplar_charsimpl_36 {
() => {
// Module: crate::exemplar_chars
// Provides: {"impl_36"}
// Dependencies: {}
impl ExemplarCharacters { # [doc = " Construct a borrowed version of this type that can be queried."] # [doc = ""] # [doc = " This avoids a potential small underlying cost per API call (ex: `contains()`) by consolidating it"] # [doc = " up front."] # [inline] pub fn as_borrowed (& self) -> ExemplarCharactersBorrowed < '_ > { ExemplarCharactersBorrowed { data : self . data . get () , } } }
};
}
