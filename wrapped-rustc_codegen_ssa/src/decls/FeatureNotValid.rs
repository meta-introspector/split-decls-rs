macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! FeatureNotValid {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_feature_not_valid)] pub (crate) struct FeatureNotValid < 'a > { pub feature : & 'a str , # [primary_span] # [label] pub span : Span , # [help] pub plus_hint : bool , }
    };
}

FeatureNotValid!()