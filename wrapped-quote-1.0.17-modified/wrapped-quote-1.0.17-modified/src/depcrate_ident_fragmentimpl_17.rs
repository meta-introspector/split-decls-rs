// Generated macro for impl_17 (impl)
macro_rules! Depcrate_ident_fragmentimpl_17 {
() => {
// Module: crate::ident_fragment
// Provides: {"impl_17"}
// Dependencies: {}
impl < T : IdentFragment + ? Sized > IdentFragment for & T { fn span (& self) -> Option < Span > { < T as IdentFragment > :: span (* self) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { IdentFragment :: fmt (* self , f) } }
};
}
