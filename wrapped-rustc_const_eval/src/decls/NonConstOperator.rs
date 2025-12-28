macro_rules! deps {
    () => {
        ConsiderDereferencing!();
    };
}

macro_rules! NonConstOperator {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (const_eval_non_const_operator , code = E0015)] pub struct NonConstOperator { # [primary_span] pub span : Span , pub kind : ConstContext , # [subdiagnostic] pub sugg : Option < ConsiderDereferencing > , pub non_or_conditionally : & 'static str , }
    };
}

NonConstOperator!();