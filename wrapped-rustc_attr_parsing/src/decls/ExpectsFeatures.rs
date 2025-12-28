macro_rules! ExpectsFeatures {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_expects_features)] pub (crate) struct ExpectsFeatures { # [primary_span] pub span : Span , pub name : String , }
    };
}

ExpectsFeatures!();