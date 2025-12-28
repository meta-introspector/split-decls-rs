macro_rules! ConcatBytesNonU8 {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_non_u8)] pub (crate) struct ConcatBytesNonU8 { # [primary_span] pub (crate) span : Span , }
    };
}

ConcatBytesNonU8!()