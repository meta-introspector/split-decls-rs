macro_rules! NonConstQuestionFromResidual {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_question_from_residual , code = E0015)] pub struct NonConstQuestionFromResidual < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
    };
}

NonConstQuestionFromResidual!();