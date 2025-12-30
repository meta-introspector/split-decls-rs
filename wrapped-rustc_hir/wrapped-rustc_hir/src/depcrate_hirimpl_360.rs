// Generated macro for impl_360 (impl)
macro_rules! Depcrate_hirimpl_360 {
() => {
// Module: crate::hir
// Provides: {"impl_360"}
// Dependencies: {}
impl < 'hir > Ty < 'hir , AmbigArg > { pub fn peel_refs (& self) -> & Ty < 'hir > { let mut final_ty = self . as_unambig_ty () ; while let TyKind :: Ref (_ , MutTy { ty , .. }) = & final_ty . kind { final_ty = ty ; } final_ty } }
};
}
