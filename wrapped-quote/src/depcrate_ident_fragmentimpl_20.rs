// Generated macro for impl_20 (impl)
macro_rules! Depcrate_ident_fragmentimpl_20 {
() => {
// Module: crate::ident_fragment
// Provides: {"impl_20"}
// Dependencies: {}
impl IdentFragment for Ident { fn span (& self) -> Option < Span > { Some (self . span ()) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let id = self . to_string () ; if let Some (id) = id . strip_prefix ("r#") { fmt :: Display :: fmt (id , f) } else { fmt :: Display :: fmt (& id [..] , f) } } }
};
}
