macro_rules! PanicNonStrErr {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_panic_non_str)] pub (crate) struct PanicNonStrErr { # [primary_span] pub span : Span , }
    };
}

PanicNonStrErr!()