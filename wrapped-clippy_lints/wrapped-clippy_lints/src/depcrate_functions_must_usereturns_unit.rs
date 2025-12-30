// Generated macro for returns_unit (function)
macro_rules! Depcrate_functions_must_usereturns_unit {
() => {
// Module: crate::functions::must_use
// Provides: {"returns_unit"}
// Dependencies: {}
fn returns_unit (decl : & hir :: FnDecl < '_ >) -> bool { match decl . output { hir :: FnRetTy :: DefaultReturn (_) => true , hir :: FnRetTy :: Return (ty) => match ty . kind { hir :: TyKind :: Tup (tys) => tys . is_empty () , hir :: TyKind :: Never => true , _ => false , } , } }
};
}
