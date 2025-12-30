// Generated macro for impl_219 (impl)
macro_rules! Depcrate_escapeimpl_219 {
() => {
// Module: crate::escape
// Provides: {"impl_219"}
// Dependencies: {}
impl std :: error :: Error for EscapeError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Self :: InvalidCharRef (e) => Some (e) , _ => None , } } }
};
}
