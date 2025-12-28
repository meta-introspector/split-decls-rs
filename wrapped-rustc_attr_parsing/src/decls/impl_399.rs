macro_rules! deps {
    () => {
        AllowedTargets!();
        Policy!();
        AllowedResult!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl AllowedTargets { pub (crate) fn is_allowed (& self , target : Target) -> AllowedResult { match self { AllowedTargets :: AllowList (list) => { if list . contains (& Policy :: Allow (target)) { AllowedResult :: Allowed } else if list . contains (& Policy :: Warn (target)) { AllowedResult :: Warn } else { AllowedResult :: Error } } AllowedTargets :: AllowListWarnRest (list) => { if list . contains (& Policy :: Allow (target)) { AllowedResult :: Allowed } else if list . contains (& Policy :: Error (target)) { AllowedResult :: Error } else { AllowedResult :: Warn } } } } pub (crate) fn allowed_targets (& self) -> Vec < Target > { match self { AllowedTargets :: AllowList (list) => list , AllowedTargets :: AllowListWarnRest (list) => list , } . iter () . filter_map (| target | match target { Policy :: Allow (target) => Some (* target) , Policy :: Warn (_) => None , Policy :: Error (_) => None , }) . collect () } }
    };
}

impl_399!()