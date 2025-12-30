// Generated macro for impl_19 (impl)
macro_rules! Depcrate_ident_fragmentimpl_19 {
() => {
// Module: crate::ident_fragment
// Provides: {"impl_19"}
// Dependencies: {}
impl < T : IdentFragment + ? Sized > IdentFragment for & mut T { fn span (& self) -> Option < Span > { < T as IdentFragment > :: span (* self) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { IdentFragment :: fmt (* self , f) } }
};
}
