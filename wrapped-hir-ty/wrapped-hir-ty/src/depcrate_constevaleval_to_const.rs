// Generated macro for eval_to_const (function)
macro_rules! Depcrate_constevaleval_to_const {
() => {
// Module: crate::consteval
// Provides: {"eval_to_const"}
// Dependencies: {}
pub (crate) fn eval_to_const < 'db > (expr : ExprId , ctx : & mut InferenceContext < '_ , 'db >) -> Const < 'db > { let infer = ctx . fixme_resolve_all_clone () ; fn has_closure (body : & Body , expr : ExprId) -> bool { if matches ! (body [expr] , Expr :: Closure { .. }) { return true ; } let mut r = false ; body . walk_child_exprs (expr , | idx | r |= has_closure (body , idx)) ; r } if has_closure (ctx . body , expr) { return unknown_const (infer [expr]) ; } if let Expr :: Path (p) = & ctx . body [expr] { let mut ctx = TyLoweringContext :: new (ctx . db , & ctx . resolver , ctx . body , ctx . generic_def , LifetimeElisionKind :: Infer ,) ; if let Some (c) = ctx . path_to_const (p) { return c ; } } if let Ok (mir_body) = lower_to_mir (ctx . db , ctx . owner , ctx . body , & infer , expr) && let Ok ((Ok (result) , _)) = interpret_mir (ctx . db , Arc :: new (mir_body) , true , None) { return result ; } unknown_const (infer [expr]) }
};
}
