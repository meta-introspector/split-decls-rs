// Generated macro for stmt_source_span (function)
macro_rules! Depcrate_loops_never_loopstmt_source_span {
() => {
// Module: crate::loops::never_loop
// Provides: {"stmt_source_span"}
// Dependencies: {}
fn stmt_source_span (stmt : & Stmt < '_ >) -> Span { let call_span = stmt . span . source_callsite () ; if stmt . span == call_span { return call_span ; } if let StmtKind :: Expr (..) = stmt . kind { return call_span ; } call_span . with_hi (call_span . hi () + BytePos (1)) }
};
}
