macro_rules! EagerlyNormalizeConsts {
    () => {
        struct EagerlyNormalizeConsts < 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , }
    };
}

EagerlyNormalizeConsts!()