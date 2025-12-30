// Generated macro for fn_def_id_and_span_from_body (function)
macro_rules! Depcrate_non_std_lazy_staticsfn_def_id_and_span_from_body {
() => {
// Module: crate::non_std_lazy_statics
// Provides: {"fn_def_id_and_span_from_body"}
// Dependencies: {}
# [doc = " Returns the `DefId` and `Span` of the callee if the given expression is a function call."] # [doc = ""] # [doc = " NB: Modified from [`clippy_utils::fn_def_id`], to support calling in an static `Item`'s body."] fn fn_def_id_and_span_from_body (cx : & LateContext < '_ > , expr : & Expr < '_ > , body_id : BodyId) -> Option < (DefId , Span) > { let typeck = cx . tcx . typeck_body (body_id) ; match & expr . kind { ExprKind :: Call (Expr { kind : ExprKind :: Path (qpath) , hir_id : path_hir_id , span , .. } , .. ,) => { if let Res :: Def (DefKind :: Fn | DefKind :: Ctor (..) | DefKind :: AssocFn , id) = typeck . qpath_res (qpath , * path_hir_id) { Some ((id , * span)) } else { None } } , _ => None , } }
};
}
