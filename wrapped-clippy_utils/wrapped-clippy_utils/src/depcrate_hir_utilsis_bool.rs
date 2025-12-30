// Generated macro for is_bool (function)
macro_rules! Depcrate_hir_utilsis_bool {
() => {
// Module: crate::hir_utils
// Provides: {"is_bool"}
// Dependencies: {}
pub fn is_bool (ty : & Ty < '_ >) -> bool { if let TyKind :: Path (QPath :: Resolved (_ , path)) = ty . kind { matches ! (path . res , Res :: PrimTy (PrimTy :: Bool)) } else { false } }
};
}
