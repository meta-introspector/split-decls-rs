// Generated macro for pat_same_as_expr (function)
macro_rules! Depcrate_matches_needless_matchpat_same_as_expr {
() => {
// Module: crate::matches::needless_match
// Provides: {"pat_same_as_expr"}
// Dependencies: {}
fn pat_same_as_expr (pat : & Pat < '_ > , expr : & Expr < '_ >) -> bool { match (& pat . kind , & expr . kind) { (PatKind :: TupleStruct (QPath :: Resolved (_ , path) , tuple_params , _) , ExprKind :: Call (call_expr , call_params)) => { if let ExprKind :: Path (QPath :: Resolved (_ , call_path)) = call_expr . kind { return over (path . segments , call_path . segments , | pat_seg , call_seg | { pat_seg . ident . name == call_seg . ident . name }) && same_non_ref_symbols (tuple_params , call_params) ; } } , (PatKind :: Binding (annot , _ , pat_ident , _) , ExprKind :: Path (QPath :: Resolved (_ , Path { segments : [first_seg , ..] , .. } ,)) ,) => { return ! matches ! (annot , BindingMode (ByRef :: Yes (..) , _)) && pat_ident . name == first_seg . ident . name ; } , (PatKind :: Expr (PatExpr { kind : PatExprKind :: Path (QPath :: Resolved (_ , p_path)) , .. }) , ExprKind :: Path (QPath :: Resolved (_ , e_path)) ,) => { return over (p_path . segments , e_path . segments , | p_seg , e_seg | { p_seg . ident . name == e_seg . ident . name }) ; } , (PatKind :: Expr (pat_expr_expr) , ExprKind :: Lit (expr_spanned)) => { if let PatExprKind :: Lit { lit : pat_spanned , negated : false , } = & pat_expr_expr . kind { return pat_spanned . node == expr_spanned . node ; } } , _ => { } , } false }
};
}
