// Generated macro for impl_7121 (impl)
macro_rules! Depcrate_methodsimpl_7121 {
() => {
// Module: crate::methods
// Provides: {"impl_7121"}
// Dependencies: {}
impl OutType { fn matches (self , ty : & hir :: FnRetTy < '_ >) -> bool { let is_unit = | ty : & hir :: Ty < '_ > | matches ! (ty . kind , hir :: TyKind :: Tup (& [])) ; match (self , ty) { (Self :: Unit , & hir :: FnRetTy :: DefaultReturn (_)) => true , (Self :: Unit , & hir :: FnRetTy :: Return (ty)) if is_unit (ty) => true , (Self :: Bool , & hir :: FnRetTy :: Return (ty)) if is_bool (ty) => true , (Self :: Any , & hir :: FnRetTy :: Return (ty)) if ! is_unit (ty) => true , (Self :: Ref , & hir :: FnRetTy :: Return (ty)) => matches ! (ty . kind , hir :: TyKind :: Ref (_ , _)) , _ => false , } } }
};
}
