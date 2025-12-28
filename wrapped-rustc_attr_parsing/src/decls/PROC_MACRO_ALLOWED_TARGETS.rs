macro_rules! deps {
    () => {
        AllowedTargets!();
    };
}

macro_rules! PROC_MACRO_ALLOWED_TARGETS {
    () => {
        deps!();
        const PROC_MACRO_ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Warn (Target :: Crate) , Warn (Target :: MacroCall)]) ;
    };
}

PROC_MACRO_ALLOWED_TARGETS!()