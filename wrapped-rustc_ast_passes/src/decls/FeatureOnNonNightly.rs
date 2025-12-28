macro_rules! deps {
    () => {
        StableFeature!();
    };
}

macro_rules! FeatureOnNonNightly {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_passes_feature_on_non_nightly , code = E0554)] pub (crate) struct FeatureOnNonNightly { # [primary_span] pub span : Span , pub channel : & 'static str , # [subdiagnostic] pub stable_features : Vec < StableFeature > , # [suggestion (code = "" , applicability = "machine-applicable")] pub sugg : Option < Span > , }
    };
}

FeatureOnNonNightly!()