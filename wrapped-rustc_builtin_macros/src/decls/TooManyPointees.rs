macro_rules! TooManyPointees {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_too_many_pointees , code = E0802)] struct TooManyPointees { # [primary_span] one : Span , # [label] another : Span , }
    };
}

TooManyPointees!();