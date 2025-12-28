macro_rules! deps {
    () => {
        MissingFeatures!();
    };
}

macro_rules! TargetFeatureDisableOrEnable {
    () => {
        deps!();
        pub struct TargetFeatureDisableOrEnable < 'a > { pub features : & 'a [& 'a str] , pub span : Option < Span > , pub missing_features : Option < MissingFeatures > , }
    };
}

TargetFeatureDisableOrEnable!();