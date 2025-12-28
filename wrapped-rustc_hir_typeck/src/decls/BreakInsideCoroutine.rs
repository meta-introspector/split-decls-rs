macro_rules! BreakInsideCoroutine {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_break_inside_coroutine , code = E0267)] pub (crate) struct BreakInsideCoroutine < 'a > { # [primary_span] # [label] pub span : Span , # [label (hir_typeck_coroutine_label)] pub coroutine_span : Span , pub name : & 'a str , pub kind : & 'a str , pub source : & 'a str , }
    };
}

BreakInsideCoroutine!()