// Generated macro for is_never_return (function)
macro_rules! Depcrate_loops_infinite_loopis_never_return {
() => {
// Module: crate::loops::infinite_loop
// Provides: {"is_never_return"}
// Dependencies: {}
# [doc = " Return `true` if the given [`FnRetTy`] is never (!)."] # [doc = ""] # [doc = " Note: This function also take care of return type of async fn,"] # [doc = " as the actual type is behind an [`OpaqueDef`](TyKind::OpaqueDef)."] fn is_never_return (ret_ty : FnRetTy < '_ >) -> bool { let FnRetTy :: Return (hir_ty) = ret_ty else { return false } ; match hir_ty . kind { TyKind :: Never => true , TyKind :: OpaqueDef (hir :: OpaqueTy { origin : hir :: OpaqueTyOrigin :: AsyncFn { .. } , bounds , .. }) => { if let Some (trait_ref) = bounds . iter () . find_map (| b | b . trait_ref ()) && let Some (segment) = trait_ref . path . segments . iter () . find (| seg | seg . ident . name == sym :: future_trait) && let Some (args) = segment . args && let Some (cst_kind) = args . constraints . iter () . find_map (| cst | (cst . ident . name == sym :: Output) . then_some (cst . kind)) && let hir :: AssocItemConstraintKind :: Equality { term : hir :: Term :: Ty (ty) , } = cst_kind { matches ! (ty . kind , TyKind :: Never) } else { false } } , _ => false , } }
};
}
