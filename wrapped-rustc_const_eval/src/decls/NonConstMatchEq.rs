macro_rules! NonConstMatchEq {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_match_eq , code = E0015)] # [note] pub struct NonConstMatchEq < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
    };
}

NonConstMatchEq!();