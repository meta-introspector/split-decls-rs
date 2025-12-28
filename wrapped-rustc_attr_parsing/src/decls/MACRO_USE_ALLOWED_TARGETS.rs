macro_rules! deps {
    () => {
        AllowedTargets!();
    };
}

macro_rules! MACRO_USE_ALLOWED_TARGETS {
    () => {
        deps!();
        const MACRO_USE_ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Mod) , Allow (Target :: ExternCrate) , Allow (Target :: Crate) , Error (Target :: WherePredicate) ,]) ;
    };
}

MACRO_USE_ALLOWED_TARGETS!()