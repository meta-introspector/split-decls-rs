// Generated macro for is_unwrap_or_expect_call (function)
macro_rules! Depcrate_unwrap_in_resultis_unwrap_or_expect_call {
() => {
// Module: crate::unwrap_in_result
// Provides: {"is_unwrap_or_expect_call"}
// Dependencies: {}
fn is_unwrap_or_expect_call (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < (OptionOrResult , Symbol) > { if let ExprKind :: Call (func , _) = expr . kind && let ExprKind :: Path (QPath :: TypeRelative (hir_ty , PathSegment { ident : Ident { name : name @ (sym :: unwrap | sym :: expect) , .. } , .. } ,)) = func . kind { is_option_or_result (cx , cx . typeck_results () . node_type (hir_ty . hir_id)) . map (| oor | (oor , * name)) } else if let ExprKind :: MethodCall (PathSegment { ident : Ident { name : name @ (sym :: unwrap | sym :: expect) , .. } , .. } , recv , _ , _ ,) = expr . kind { is_option_or_result (cx , cx . typeck_results () . expr_ty_adjusted (recv)) . map (| oor | (oor , * name)) } else { None } }
};
}
