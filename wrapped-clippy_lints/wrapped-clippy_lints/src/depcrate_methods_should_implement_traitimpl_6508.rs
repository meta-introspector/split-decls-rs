// Generated macro for impl_6508 (impl)
macro_rules! Depcrate_methods_should_implement_traitimpl_6508 {
() => {
// Module: crate::methods::should_implement_trait
// Provides: {"impl_6508"}
// Dependencies: {}
impl OutType { fn matches (self , ty : & FnRetTy < '_ >) -> bool { let is_unit = | ty : & hir :: Ty < '_ > | matches ! (ty . kind , hir :: TyKind :: Tup (& [])) ; match (self , ty) { (Self :: Unit , & FnRetTy :: DefaultReturn (_)) => true , (Self :: Unit , & FnRetTy :: Return (ty)) if is_unit (ty) => true , (Self :: Bool , & FnRetTy :: Return (ty)) if is_bool (ty) => true , (Self :: Any , & FnRetTy :: Return (ty)) if ! is_unit (ty) => true , (Self :: Ref , & FnRetTy :: Return (ty)) => matches ! (ty . kind , hir :: TyKind :: Ref (_ , _)) , _ => false , } } }
};
}
