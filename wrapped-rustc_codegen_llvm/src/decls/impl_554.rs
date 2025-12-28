macro_rules! deps {
    () => {
        SmallVec!();
        LLVMFeature!();
        TargetFeatureFoldStrength!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        impl < 'a > LLVMFeature < 'a > { fn new (llvm_feature_name : & 'a str) -> Self { Self { llvm_feature_name , dependencies : SmallVec :: new () } } fn with_dependencies (llvm_feature_name : & 'a str , dependencies : SmallVec < [TargetFeatureFoldStrength < 'a > ; 1] > ,) -> Self { Self { llvm_feature_name , dependencies } } }
    };
}

impl_554!()