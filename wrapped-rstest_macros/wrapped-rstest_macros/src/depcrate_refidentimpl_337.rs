// Generated macro for impl_337 (impl)
macro_rules! Depcrate_refidentimpl_337 {
() => {
// Module: crate::refident
// Provides: {"impl_337"}
// Dependencies: {}
impl MaybeIdent for crate :: parse :: Attribute { fn maybe_ident (& self) -> Option < & Ident > { use crate :: parse :: Attribute :: * ; match self { Attr (ident) | Tagged (ident , _) | Type (ident , _) => Some (ident) , } } }
};
}
