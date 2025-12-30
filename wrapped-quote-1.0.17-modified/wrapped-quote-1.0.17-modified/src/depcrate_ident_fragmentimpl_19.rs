// Generated macro for impl_19 (impl)
macro_rules! Depcrate_ident_fragmentimpl_19 {
() => {
// Module: crate::ident_fragment
// Provides: {"impl_19"}
// Dependencies: {}
impl IdentFragment for Ident { fn span (& self) -> Option < Span > { Some (self . span ()) } fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let id = self . to_string () ; if id . starts_with ("r#") { fmt :: Display :: fmt (& id [2 ..] , f) } else { fmt :: Display :: fmt (& id [..] , f) } } }
};
}
