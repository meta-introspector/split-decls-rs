macro_rules! ConcatCStrLit {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_c_str_lit)] pub (crate) struct ConcatCStrLit { # [primary_span] pub (crate) span : Span , }
    };
}

ConcatCStrLit!()