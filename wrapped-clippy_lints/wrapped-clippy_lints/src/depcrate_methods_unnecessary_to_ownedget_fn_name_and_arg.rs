// Generated macro for get_fn_name_and_arg (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedget_fn_name_and_arg {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"get_fn_name_and_arg"}
// Dependencies: {}
fn get_fn_name_and_arg < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> Option < (Symbol , Expr < 'tcx >) > { match & expr . kind { ExprKind :: MethodCall (path , _ , [arg_expr] , ..) => Some ((path . ident . name , * arg_expr)) , ExprKind :: Call (Expr { kind : ExprKind :: Path (qpath) , hir_id : path_hir_id , .. } , [arg_expr] ,) => { if let Res :: Def (DefKind :: Fn | DefKind :: Ctor (..) | DefKind :: AssocFn , def_id) = cx . typeck_results () . qpath_res (qpath , * path_hir_id) && let Some (fn_name) = cx . tcx . opt_item_name (def_id) { Some ((fn_name , * arg_expr)) } else { None } } , _ => None , } }
};
}
