// Generated macro for check_block_return (function)
macro_rules! Depcrate_returns_needless_returncheck_block_return {
() => {
// Module: crate::returns::needless_return
// Provides: {"check_block_return"}
// Dependencies: {}
fn check_block_return < 'tcx > (cx : & LateContext < 'tcx > , expr_kind : & ExprKind < 'tcx > , sp : Span , mut semi_spans : Vec < Span >) { if let ExprKind :: Block (block , _) = expr_kind { if let Some (block_expr) = block . expr { check_final_expr (cx , block_expr , semi_spans , RetReplacement :: Empty , None) ; } else if let Some (stmt) = block . stmts . last () { if span_contains_cfg (cx , Span :: between (stmt . span , cx . sess () . source_map () . end_point (block . span) ,) ,) { return ; } match stmt . kind { StmtKind :: Expr (expr) => { check_final_expr (cx , expr , semi_spans , RetReplacement :: Empty , None) ; } , StmtKind :: Semi (semi_expr) => { if let Some (semi_span) = stmt . span . trim_start (semi_expr . span) { let semi_span_to_remove = span_find_starting_semi (cx . sess () . source_map () , semi_span . with_hi (sp . hi ())) ; semi_spans . push (semi_span_to_remove) ; } check_final_expr (cx , semi_expr , semi_spans , RetReplacement :: Empty , None) ; } , _ => () , } } } }
};
}
