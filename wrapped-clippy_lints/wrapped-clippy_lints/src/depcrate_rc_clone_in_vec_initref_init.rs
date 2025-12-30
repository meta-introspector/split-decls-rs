// Generated macro for ref_init (function)
macro_rules! Depcrate_rc_clone_in_vec_initref_init {
() => {
// Module: crate::rc_clone_in_vec_init
// Provides: {"ref_init"}
// Dependencies: {}
# [doc = " Checks whether the given `expr` is a call to `Arc::new`, `Rc::new`, or evaluates to a `Weak`"] fn ref_init (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < (Symbol , Span) > { if let ExprKind :: Call (func , _args) = expr . kind && let ExprKind :: Path (ref func_path @ QPath :: TypeRelative (ty , _)) = func . kind && let TyKind :: Path (ref ty_path) = ty . kind && let Some (def_id) = cx . qpath_res (ty_path , ty . hir_id) . opt_def_id () { if last_path_segment (func_path) . ident . name == sym :: new && let Some (symbol) = cx . tcx . get_diagnostic_name (def_id) . filter (| symbol | symbol == & sym :: Arc || symbol == & sym :: Rc) { return Some ((symbol , func . span)) ; } if let ty :: Adt (adt , _) = * cx . typeck_results () . expr_ty (expr) . kind () && matches ! (cx . tcx . get_diagnostic_name (adt . did ()) , Some (sym :: RcWeak | sym :: ArcWeak)) { return Some ((sym :: Weak , func . span)) ; } } None }
};
}
