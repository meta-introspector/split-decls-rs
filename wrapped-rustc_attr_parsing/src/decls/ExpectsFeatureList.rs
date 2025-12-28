macro_rules! ExpectsFeatureList {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_expects_feature_list)] pub (crate) struct ExpectsFeatureList { # [primary_span] pub span : Span , pub name : String , }
    };
}

ExpectsFeatureList!()