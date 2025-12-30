// Generated macro for impl_340 (impl)
macro_rules! Depcrate_refidentimpl_340 {
() => {
// Module: crate::refident
// Provides: {"impl_340"}
// Dependencies: {}
impl MaybePatIdent for FnArg { fn maybe_patident (& self) -> Option < & syn :: PatIdent > { match self { FnArg :: Typed (PatType { pat , .. }) => match pat . as_ref () { Pat :: Ident (ident) => Some (ident) , _ => None , } , _ => None , } } }
};
}
