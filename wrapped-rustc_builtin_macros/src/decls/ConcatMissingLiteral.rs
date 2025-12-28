macro_rules! ConcatMissingLiteral {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_concat_missing_literal)] # [note] pub (crate) struct ConcatMissingLiteral { # [primary_span] pub (crate) spans : Vec < Span > , }
    };
}

ConcatMissingLiteral!()