macro_rules! ConcatBytestr {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytestr)] pub (crate) struct ConcatBytestr { # [primary_span] pub (crate) span : Span , }
    };
}

ConcatBytestr!()