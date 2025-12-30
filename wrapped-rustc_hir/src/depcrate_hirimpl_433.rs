// Generated macro for impl_433 (impl)
macro_rules! Depcrate_hirimpl_433 {
() => {
// Module: crate::hir
// Provides: {"impl_433"}
// Dependencies: {}
impl < 'hir > From < OwnerNode < 'hir > > for Node < 'hir > { fn from (val : OwnerNode < 'hir >) -> Self { match val { OwnerNode :: Item (n) => Node :: Item (n) , OwnerNode :: ForeignItem (n) => Node :: ForeignItem (n) , OwnerNode :: ImplItem (n) => Node :: ImplItem (n) , OwnerNode :: TraitItem (n) => Node :: TraitItem (n) , OwnerNode :: Crate (n) => Node :: Crate (n) , OwnerNode :: Synthetic => Node :: Synthetic , } } }
};
}
