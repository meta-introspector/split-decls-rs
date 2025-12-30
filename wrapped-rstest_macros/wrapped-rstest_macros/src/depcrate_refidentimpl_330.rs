// Generated macro for impl_330 (impl)
macro_rules! Depcrate_refidentimpl_330 {
() => {
// Module: crate::refident
// Provides: {"impl_330"}
// Dependencies: {}
impl MaybeIdent for FnArg { fn maybe_ident (& self) -> Option < & Ident > { match self { FnArg :: Typed (pat) => pat . maybe_ident () , _ => None , } } }
};
}
