// Generated macro for impl_216 (impl)
macro_rules! Depcrate_escapeimpl_216 {
() => {
// Module: crate::escape
// Provides: {"impl_216"}
// Dependencies: {}
impl std :: error :: Error for ParseCharRefError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Self :: InvalidNumber (e) => Some (e) , _ => None , } } }
};
}
