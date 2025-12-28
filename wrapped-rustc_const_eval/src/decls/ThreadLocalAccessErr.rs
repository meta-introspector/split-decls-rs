macro_rules! ThreadLocalAccessErr {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_thread_local_access , code = E0625)] pub (crate) struct ThreadLocalAccessErr { # [primary_span] pub span : Span , }
    };
}

ThreadLocalAccessErr!()