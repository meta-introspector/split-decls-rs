macro_rules! deps {
    () => {
        TargetFeatureFoldStrength!();
        SmallVec!();
    };
}

macro_rules! LLVMFeature {
    () => {
        deps!();
        pub (crate) struct LLVMFeature < 'a > { llvm_feature_name : & 'a str , dependencies : SmallVec < [TargetFeatureFoldStrength < 'a > ; 1] > , }
    };
}

LLVMFeature!()