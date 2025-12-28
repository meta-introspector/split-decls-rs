macro_rules! InvalidReprAlignNeedArg {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_repr_align_need_arg , code = E0589)] pub (crate) struct InvalidReprAlignNeedArg { # [primary_span] # [suggestion (code = "align(...)" , applicability = "has-placeholders")] pub span : Span , }
    };
}

InvalidReprAlignNeedArg!();