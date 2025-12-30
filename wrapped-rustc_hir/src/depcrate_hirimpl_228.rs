// Generated macro for impl_228 (impl)
macro_rules! Depcrate_hirimpl_228 {
() => {
// Module: crate::hir
// Provides: {"impl_228"}
// Dependencies: {}
impl GenericBound < '_ > { pub fn trait_ref (& self) -> Option < & TraitRef < '_ > > { match self { GenericBound :: Trait (data) => Some (& data . trait_ref) , _ => None , } } pub fn span (& self) -> Span { match self { GenericBound :: Trait (t , ..) => t . span , GenericBound :: Outlives (l) => l . ident . span , GenericBound :: Use (_ , span) => * span , } } }
};
}
