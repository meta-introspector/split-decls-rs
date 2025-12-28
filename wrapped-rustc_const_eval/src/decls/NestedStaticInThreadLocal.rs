macro_rules! NestedStaticInThreadLocal {
    () => {
        # [derive (Diagnostic)] # [diag (const_eval_nested_static_in_thread_local)] pub (crate) struct NestedStaticInThreadLocal { # [primary_span] pub span : Span , }
    };
}

NestedStaticInThreadLocal!();