// Generated macro for impl_107 (impl)
macro_rules! Depcrate_builtinimpl_107 {
() => {
// Module: crate::builtin
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MutableTransmutes { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) { if let Some ((& ty :: Ref (_ , _ , from_mutbl) , & ty :: Ref (_ , _ , to_mutbl))) = get_transmute_from_to (cx , expr) . map (| (ty1 , ty2) | (ty1 . kind () , ty2 . kind ())) { if from_mutbl < to_mutbl { cx . emit_span_lint (MUTABLE_TRANSMUTES , expr . span , BuiltinMutablesTransmutes) ; } } fn get_transmute_from_to < 'tcx > (cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ > ,) -> Option < (Ty < 'tcx > , Ty < 'tcx >) > { let def = if let hir :: ExprKind :: Path (ref qpath) = expr . kind { cx . qpath_res (qpath , expr . hir_id) } else { return None ; } ; if let Res :: Def (DefKind :: Fn , did) = def { if ! def_id_is_transmute (cx , did) { return None ; } let sig = cx . typeck_results () . node_type (expr . hir_id) . fn_sig (cx . tcx) ; let from = sig . inputs () . skip_binder () [0] ; let to = sig . output () . skip_binder () ; return Some ((from , to)) ; } None } fn def_id_is_transmute (cx : & LateContext < '_ > , def_id : DefId) -> bool { cx . tcx . is_intrinsic (def_id , sym :: transmute) } } }
};
}
