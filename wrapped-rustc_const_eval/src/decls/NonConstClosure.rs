macro_rules! deps {
    () => {
        NonConstClosureNote!();
    };
}

macro_rules! NonConstClosure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (const_eval_non_const_closure , code = E0015)] pub struct NonConstClosure { # [primary_span] pub span : Span , pub kind : ConstContext , # [subdiagnostic] pub note : Option < NonConstClosureNote > , pub non_or_conditionally : & 'static str , }
    };
}

NonConstClosure!();