macro_rules! ConcatBytesBadRepeat {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_bad_repeat)] pub (crate) struct ConcatBytesBadRepeat { # [primary_span] pub (crate) span : Span , }
    };
}

ConcatBytesBadRepeat!()