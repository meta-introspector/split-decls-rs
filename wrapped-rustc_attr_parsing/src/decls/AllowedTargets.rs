macro_rules! deps {
    () => {
        Policy!();
    };
}

macro_rules! AllowedTargets {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) enum AllowedTargets { AllowList (& 'static [Policy]) , AllowListWarnRest (& 'static [Policy]) , }
    };
}

AllowedTargets!();