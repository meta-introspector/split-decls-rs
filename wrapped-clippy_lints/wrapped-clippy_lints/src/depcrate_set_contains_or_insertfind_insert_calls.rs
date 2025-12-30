// Generated macro for find_insert_calls (function)
macro_rules! Depcrate_set_contains_or_insertfind_insert_calls {
() => {
// Module: crate::set_contains_or_insert
// Provides: {"find_insert_calls"}
// Dependencies: {}
fn find_insert_calls < 'tcx > (cx : & LateContext < 'tcx > , contains_expr : & OpExpr < 'tcx > , expr : & 'tcx Expr < '_ > ,) -> Option < OpExpr < 'tcx > > { for_each_expr (cx , expr , | e | { if let Some ((insert_expr , _)) = try_parse_op_call (cx , e , sym :: insert) && SpanlessEq :: new (cx) . eq_expr (contains_expr . receiver , insert_expr . receiver) && SpanlessEq :: new (cx) . eq_expr (contains_expr . value , insert_expr . value) { ControlFlow :: Break (insert_expr) } else { ControlFlow :: Continue (()) } }) }
};
}
