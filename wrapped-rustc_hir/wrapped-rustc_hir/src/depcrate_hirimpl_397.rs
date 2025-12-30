// Generated macro for impl_397 (impl)
macro_rules! Depcrate_hirimpl_397 {
() => {
// Module: crate::hir
// Provides: {"impl_397"}
// Dependencies: {}
impl TraitRef < '_ > { # [doc = " Gets the `DefId` of the referenced trait. It _must_ actually be a trait or trait alias."] pub fn trait_def_id (& self) -> Option < DefId > { match self . path . res { Res :: Def (DefKind :: Trait | DefKind :: TraitAlias , did) => Some (did) , Res :: Err => None , res => panic ! ("{res:?} did not resolve to a trait or trait alias") , } } }
};
}
