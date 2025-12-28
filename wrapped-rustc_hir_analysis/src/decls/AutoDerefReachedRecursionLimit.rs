macro_rules! AutoDerefReachedRecursionLimit {
    () => {
        # [derive (Diagnostic)] # [help] # [diag (hir_analysis_auto_deref_reached_recursion_limit , code = E0055)] pub (crate) struct AutoDerefReachedRecursionLimit < 'a > { # [primary_span] # [label] pub span : Span , pub ty : Ty < 'a > , pub suggested_limit : Limit , pub crate_name : Symbol , }
    };
}

AutoDerefReachedRecursionLimit!()