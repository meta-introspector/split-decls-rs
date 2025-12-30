// Generated macro for span_lint (function)
macro_rules! Depcrate_use_selfspan_lint {
() => {
// Module: crate::use_self
// Provides: {"span_lint"}
// Dependencies: {}
fn span_lint (cx : & LateContext < '_ > , span : Span) { span_lint_and_sugg (cx , USE_SELF , span , "unnecessary structure name repetition" , "use the applicable keyword" , "Self" . to_owned () , Applicability :: MachineApplicable ,) ; }
};
}
