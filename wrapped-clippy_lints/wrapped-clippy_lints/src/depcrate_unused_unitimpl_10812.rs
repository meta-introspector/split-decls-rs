// Generated macro for impl_10812 (impl)
macro_rules! Depcrate_unused_unitimpl_10812 {
() => {
// Module: crate::unused_unit
// Provides: {"impl_10812"}
// Dependencies: {}
impl EarlyLintPass for UnusedUnit { # [doc = " Check for unit expressions in blocks. This is left in the early pass because some macros"] # [doc = " expand its inputs as-is, making it invisible to the late pass. See #4076."] fn check_block (& mut self , cx : & EarlyContext < '_ > , block : & Block) { if let Some (stmt) = block . stmts . last () && let StmtKind :: Expr (expr) = & stmt . kind && let rustc_ast :: ExprKind :: Tup (inner) = & expr . kind && inner . is_empty () && let ctxt = block . span . ctxt () && stmt . span . ctxt () == ctxt && expr . span . ctxt () == ctxt && expr . attrs . is_empty () { let sp = expr . span ; span_lint_and_sugg (cx , UNUSED_UNIT , sp , "unneeded unit expression" , "remove the final `()`" , String :: new () , Applicability :: MachineApplicable ,) ; } } }
};
}
