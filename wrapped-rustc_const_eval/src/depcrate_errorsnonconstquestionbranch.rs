// Generated macro for NonConstQuestionBranch (struct)
macro_rules! Depcrate_errorsNonConstQuestionBranch {
() => {
// Module: crate::errors
// Provides: {"NonConstQuestionBranch"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_non_const_question_branch , code = E0015)] pub struct NonConstQuestionBranch < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
};
}
