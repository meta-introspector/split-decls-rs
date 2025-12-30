// Generated macro for report_lint (function)
macro_rules! Depcrate_missing_asserts_for_indexingreport_lint {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"report_lint"}
// Dependencies: {}
fn report_lint < F > (cx : & LateContext < '_ > , full_span : Span , msg : & 'static str , indexes : & [Span] , f : F) where F : FnOnce (& mut Diag < '_ , () >) , { span_lint_and_then (cx , MISSING_ASSERTS_FOR_INDEXING , full_span , msg , | diag | { f (diag) ; for span in indexes { diag . span_note (* span , "slice indexed here") ; } diag . note ("asserting the length before indexing will elide bounds checks") ; }) ; }
};
}
