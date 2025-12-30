// Generated macro for is_assoc_fn_without_type_instance (function)
macro_rules! Depcrate_operators_identity_opis_assoc_fn_without_type_instance {
() => {
// Module: crate::operators::identity_op
// Provides: {"is_assoc_fn_without_type_instance"}
// Dependencies: {}
# [doc = " Check if the expression is an associated function without a type instance."] # [doc = " Example:"] # [doc = " ```"] # [doc = " trait Def {"] # [doc = "     fn def() -> Self;"] # [doc = " }"] # [doc = " impl Def for usize {"] # [doc = "     fn def() -> Self {"] # [doc = "         0"] # [doc = "     }"] # [doc = " }"] # [doc = " fn test() {"] # [doc = "     let _ = 0usize + &Default::default();"] # [doc = "     let _ = 0usize + &Def::def();"] # [doc = " }"] # [doc = " ```"] fn is_assoc_fn_without_type_instance < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> bool { if let ExprKind :: Call (func , _) = peel_hir_expr_refs (expr) . 0 . kind && let ExprKind :: Path (QPath :: Resolved (None , Path { res : Res :: Def (DefKind :: AssocFn , def_id) , .. } ,)) = func . kind && let output_ty = cx . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () . output () && let ty :: Param (ty :: ParamTy { name : kw :: SelfUpper , .. }) = output_ty . kind () { return true ; } false }
};
}
