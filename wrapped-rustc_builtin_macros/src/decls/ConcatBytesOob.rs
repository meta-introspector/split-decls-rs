macro_rules! ConcatBytesOob {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_oob)] pub (crate) struct ConcatBytesOob { # [primary_span] pub (crate) span : Span , }
    };
}

ConcatBytesOob!()