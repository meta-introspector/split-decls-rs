// Generated macro for impl_341 (impl)
macro_rules! Depcrate_refidentimpl_341 {
() => {
// Module: crate::refident
// Provides: {"impl_341"}
// Dependencies: {}
impl MaybePatIdent for Pat { fn maybe_patident (& self) -> Option < & syn :: PatIdent > { match self { Pat :: Ident (ident) => Some (ident) , _ => None , } } }
};
}
