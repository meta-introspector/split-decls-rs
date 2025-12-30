// Generated macro for report_lint (function)
macro_rules! Depcrate_loops_manual_while_let_somereport_lint {
() => {
// Module: crate::loops::manual_while_let_some
// Provides: {"report_lint"}
// Dependencies: {}
fn report_lint (cx : & LateContext < '_ > , pop_span : Span , pop_stmt_kind : PopStmt < '_ > , loop_span : Span , receiver_span : Span) { span_lint_and_then (cx , MANUAL_WHILE_LET_SOME , pop_span , "you seem to be trying to pop elements from a `Vec` in a loop" , | diag | { let (pat , pop_replacement) = match pop_stmt_kind { PopStmt :: Local (pat) => (snippet (cx , pat . span , "..") , String :: new ()) , PopStmt :: Anonymous => (Cow :: Borrowed ("element") , "element" . into ()) , } ; let loop_replacement = format ! ("while let Some({}) = {}.pop()" , pat , snippet (cx , receiver_span , "..")) ; diag . multipart_suggestion ("consider using a `while..let` loop" , vec ! [(loop_span , loop_replacement) , (pop_span , pop_replacement)] , Applicability :: MachineApplicable ,) ; } ,) ; }
};
}
