macro_rules! PatternFnPointer {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_pattern_in_fn_pointer , code = E0561)] pub (crate) struct PatternFnPointer { # [primary_span] pub span : Span , }
    };
}

PatternFnPointer!();