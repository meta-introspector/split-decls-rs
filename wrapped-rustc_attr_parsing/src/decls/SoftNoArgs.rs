macro_rules! SoftNoArgs {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_soft_no_args)] pub (crate) struct SoftNoArgs { # [primary_span] pub span : Span , }
    };
}

SoftNoArgs!()