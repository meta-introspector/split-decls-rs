macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! UnstableFeatureUsage {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (driver_impl_unstable_feature_usage)] pub (crate) struct UnstableFeatureUsage { pub error : Box < dyn Error > , }
    };
}

UnstableFeatureUsage!()