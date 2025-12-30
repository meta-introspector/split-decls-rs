// Generated macro for impl_331 (impl)
macro_rules! Depcrate_diagnostics_conflict_errorsimpl_331 {
() => {
// Module: crate::diagnostics::conflict_errors
// Provides: {"impl_331"}
// Dependencies: {}
impl < 'v > Visitor < 'v > for ReferencedStatementsVisitor < '_ > { type Result = ControlFlow < () > ; fn visit_stmt (& mut self , s : & 'v hir :: Stmt < 'v >) -> Self :: Result { match s . kind { hir :: StmtKind :: Semi (expr) if self . 0 . contains (& expr . span) => ControlFlow :: Break (()) , _ => ControlFlow :: Continue (()) , } } }
};
}
