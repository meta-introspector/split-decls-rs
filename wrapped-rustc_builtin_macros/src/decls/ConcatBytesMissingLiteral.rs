macro_rules! ConcatBytesMissingLiteral {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_missing_literal)] # [note] pub (crate) struct ConcatBytesMissingLiteral { # [primary_span] pub (crate) spans : Vec < Span > , }
    };
}

ConcatBytesMissingLiteral!();