macro_rules! TargetFeatureFoldStrength {
    () => {
        enum TargetFeatureFoldStrength < 'a > { EnableOnly (& 'a str) , Both (& 'a str) , }
    };
}

TargetFeatureFoldStrength!()