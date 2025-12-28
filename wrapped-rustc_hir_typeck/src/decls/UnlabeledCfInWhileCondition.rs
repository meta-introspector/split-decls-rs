macro_rules! UnlabeledCfInWhileCondition {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_unlabeled_cf_in_while_condition , code = E0590)] pub (crate) struct UnlabeledCfInWhileCondition < 'a > { # [primary_span] # [label] pub span : Span , pub cf_type : & 'a str , }
    };
}

UnlabeledCfInWhileCondition!();