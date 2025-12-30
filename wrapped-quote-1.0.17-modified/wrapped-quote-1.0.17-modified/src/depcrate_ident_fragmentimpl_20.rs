// Generated macro for impl_20 (impl)
macro_rules! Depcrate_ident_fragmentimpl_20 {
() => {
// Module: crate::ident_fragment
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > IdentFragment for Cow < '_ , T > where T : IdentFragment + ToOwned + ? Sized , { fn span (& self) -> Option < Span > { T :: span (self) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { T :: fmt (self , f) } }
};
}
