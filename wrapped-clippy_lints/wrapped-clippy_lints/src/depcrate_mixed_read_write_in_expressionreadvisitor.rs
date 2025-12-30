// Generated macro for ReadVisitor (struct)
macro_rules! Depcrate_mixed_read_write_in_expressionReadVisitor {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"ReadVisitor"}
// Dependencies: {}
# [doc = " A visitor that looks for reads from a variable."] struct ReadVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , # [doc = " The ID of the variable we're looking for."] var : HirId , # [doc = " The expressions where the write to the variable occurred (for reporting"] # [doc = " in the lint)."] write_expr : & 'tcx Expr < 'tcx > , # [doc = " The last (highest in the AST) expression we've checked, so we know not"] # [doc = " to recheck it."] last_expr : & 'tcx Expr < 'tcx > , }
};
}
