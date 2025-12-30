// Generated macro for impl_336 (impl)
macro_rules! Depcrate_refidentimpl_336 {
() => {
// Module: crate::refident
// Provides: {"impl_336"}
// Dependencies: {}
impl MaybeIdent for syn :: GenericParam { fn maybe_ident (& self) -> Option < & Ident > { match self { syn :: GenericParam :: Type (syn :: TypeParam { ident , .. }) | syn :: GenericParam :: Const (syn :: ConstParam { ident , .. }) => Some (ident) , syn :: GenericParam :: Lifetime (syn :: LifetimeParam { lifetime , .. }) => { Some (& lifetime . ident) } } } }
};
}
