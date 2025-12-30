// Generated macro for has_no_effect (function)
macro_rules! Depcrate_no_effecthas_no_effect {
() => {
// Module: crate::no_effect
// Provides: {"has_no_effect"}
// Dependencies: {}
fn has_no_effect (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Lit (..) | ExprKind :: Closure { .. } => true , ExprKind :: Path (..) => ! expr_ty_has_significant_drop (cx , expr) , ExprKind :: Index (a , b , _) | ExprKind :: Binary (_ , a , b) => has_no_effect (cx , a) && has_no_effect (cx , b) , ExprKind :: Array (v) | ExprKind :: Tup (v) => v . iter () . all (| val | has_no_effect (cx , val)) , ExprKind :: Repeat (inner , _) | ExprKind :: Cast (inner , _) | ExprKind :: Type (inner , _) | ExprKind :: Unary (_ , inner) | ExprKind :: Field (inner , _) | ExprKind :: AddrOf (_ , _ , inner) => has_no_effect (cx , inner) , ExprKind :: Struct (_ , fields , base) => { ! expr_ty_has_significant_drop (cx , expr) && fields . iter () . all (| field | has_no_effect (cx , field . expr)) && match & base { StructTailExpr :: None | StructTailExpr :: DefaultFields (_) => true , StructTailExpr :: Base (base) => has_no_effect (cx , base) , } } , ExprKind :: Call (callee , args) => { if let ExprKind :: Path (ref qpath) = callee . kind { if cx . typeck_results () . type_dependent_def (expr . hir_id) . is_some () { return false ; } let def_matched = matches ! (cx . qpath_res (qpath , callee . hir_id) , Res :: Def (DefKind :: Struct | DefKind :: Variant | DefKind :: Ctor (..) , ..)) ; if def_matched || is_range_literal (expr) { ! expr_ty_has_significant_drop (cx , expr) && args . iter () . all (| arg | has_no_effect (cx , arg)) } else { false } } else { false } } , _ => false , } }
};
}
