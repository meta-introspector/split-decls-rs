// Generated macro for is_visible (function)
macro_rules! Depcrate_registryis_visible {
() => {
// Module: crate::registry
// Provides: {"is_visible"}
// Dependencies: {}
pub (crate) fn is_visible (ctx : & Context < '_ > , visible : & Option < MetaVisibleFn >) -> bool { match visible { Some (f) => f (ctx) , None => true , } }
};
}
