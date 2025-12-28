macro_rules! InvalidAlignmentValue {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_alignment_value , code = E0589)] pub (crate) struct InvalidAlignmentValue { # [primary_span] pub span : Span , pub error_part : & 'static str , }
    };
}

InvalidAlignmentValue!()