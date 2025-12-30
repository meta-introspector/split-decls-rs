// Generated macro for impl_154 (impl)
macro_rules! Depcrate_astimpl_154 {
() => {
// Module: crate::ast
// Provides: {"impl_154"}
// Dependencies: {}
impl Ty { pub fn peel_refs (& self) -> & Self { let mut final_ty = self ; while let TyKind :: Ref (_ , MutTy { ty , .. }) | TyKind :: Ptr (MutTy { ty , .. }) = & final_ty . kind { final_ty = ty ; } final_ty } pub fn is_maybe_parenthesised_infer (& self) -> bool { match & self . kind { TyKind :: Infer => true , TyKind :: Paren (inner) => inner . is_maybe_parenthesised_infer () , _ => false , } } }
};
}
