macro_rules! NullOnExport {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_null_on_export , code = E0648)] pub (crate) struct NullOnExport { # [primary_span] pub span : Span , }
    };
}

NullOnExport!();