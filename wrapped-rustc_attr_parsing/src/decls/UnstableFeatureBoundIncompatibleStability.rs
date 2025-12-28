macro_rules! UnstableFeatureBoundIncompatibleStability {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_unstable_feature_bound_incompatible_stability)] # [help] pub (crate) struct UnstableFeatureBoundIncompatibleStability { # [primary_span] pub span : Span , }
    };
}

UnstableFeatureBoundIncompatibleStability!()