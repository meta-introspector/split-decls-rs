// Generated macro for check_for_unsequenced_reads (function)
macro_rules! Depcrate_mixed_read_write_in_expressioncheck_for_unsequenced_reads {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"check_for_unsequenced_reads"}
// Dependencies: {}
# [doc = " Walks up the AST from the given write expression (`vis.write_expr`) looking"] # [doc = " for reads to the same variable that are unsequenced relative to the write."] # [doc = ""] # [doc = " This means reads for which there is a common ancestor between the read and"] # [doc = " the write such that"] # [doc = ""] # [doc = " * evaluating the ancestor necessarily evaluates both the read and the write (for example, `&x`"] # [doc = "   and `|| x = 1` don't necessarily evaluate `x`), and"] # [doc = ""] # [doc = " * which one is evaluated first depends on the order of sub-expression evaluation. Blocks, `if`s,"] # [doc = "   loops, `match`es, and the short-circuiting logical operators are considered to have a defined"] # [doc = "   evaluation order."] # [doc = ""] # [doc = " When such a read is found, the lint is triggered."] fn check_for_unsequenced_reads (vis : & mut ReadVisitor < '_ , '_ >) { let mut cur_id = vis . write_expr . hir_id ; loop { let parent_id = vis . cx . tcx . parent_hir_id (cur_id) ; if parent_id == cur_id { break ; } let stop_early = match vis . cx . tcx . hir_node (parent_id) { Node :: Expr (expr) => check_expr (vis , expr) , Node :: Stmt (stmt) => check_stmt (vis , stmt) , Node :: Item (_) => { break ; } , _ => StopEarly :: KeepGoing , } ; match stop_early { StopEarly :: Stop => break , StopEarly :: KeepGoing => { } , } cur_id = parent_id ; } }
};
}
