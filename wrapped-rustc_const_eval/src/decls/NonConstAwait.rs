macro_rules! NonConstAwait {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_non_const_await , code = E0015)] pub struct NonConstAwait < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub non_or_conditionally : & 'static str , }
    };
}

NonConstAwait!()