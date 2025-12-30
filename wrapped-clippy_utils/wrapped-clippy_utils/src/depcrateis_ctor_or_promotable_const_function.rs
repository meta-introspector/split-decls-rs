// Generated macro for is_ctor_or_promotable_const_function (function)
macro_rules! Depcrateis_ctor_or_promotable_const_function {
() => {
// Module: crate
// Provides: {"is_ctor_or_promotable_const_function"}
// Dependencies: {}
# [doc = " Checks if an expression is constructing a tuple-like enum variant or struct"] pub fn is_ctor_or_promotable_const_function (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let ExprKind :: Call (fun , _) = expr . kind && let ExprKind :: Path (ref qp) = fun . kind { let res = cx . qpath_res (qp , fun . hir_id) ; return match res { Res :: Def (DefKind :: Variant | DefKind :: Ctor (..) , ..) => true , Res :: Def (_ , def_id) => cx . tcx . is_promotable_const_fn (def_id) , _ => false , } ; } false }
};
}
