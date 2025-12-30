// Generated macro for fn_def_id_with_node_args (function)
macro_rules! Depcratefn_def_id_with_node_args {
() => {
// Module: crate
// Provides: {"fn_def_id_with_node_args"}
// Dependencies: {}
# [doc = " Returns the `DefId` of the callee if the given expression is a function or method call,"] # [doc = " as well as its node args."] pub fn fn_def_id_with_node_args < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ > ,) -> Option < (DefId , GenericArgsRef < 'tcx >) > { let typeck = cx . typeck_results () ; match & expr . kind { ExprKind :: MethodCall (..) => Some ((typeck . type_dependent_def_id (expr . hir_id) ? , typeck . node_args (expr . hir_id) ,)) , ExprKind :: Call (Expr { kind : ExprKind :: Path (qpath) , hir_id : path_hir_id , .. } , .. ,) => { if let Res :: Def (DefKind :: Fn | DefKind :: Ctor (..) | DefKind :: AssocFn , id) = typeck . qpath_res (qpath , * path_hir_id) { Some ((id , typeck . node_args (* path_hir_id))) } else { None } } , _ => None , } }
};
}
