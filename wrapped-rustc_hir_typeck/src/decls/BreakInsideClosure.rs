macro_rules! BreakInsideClosure {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_break_inside_closure , code = E0267)] pub (crate) struct BreakInsideClosure < 'a > { # [primary_span] # [label] pub span : Span , # [label (hir_typeck_closure_label)] pub closure_span : Span , pub name : & 'a str , }
    };
}

BreakInsideClosure!();