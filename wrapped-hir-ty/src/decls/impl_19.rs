macro_rules! deps {
    () => {
        AutoderefCtx!();
        InferenceContextAutoderefCtx!();
        TraitEnvironment!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'db > AutoderefCtx < 'db > for InferenceContextAutoderefCtx < '_ , '_ , 'db > { # [inline] fn infcx (& self) -> & InferCtxt < 'db > { & self . 0 . table . infer_ctxt } # [inline] fn env (& self) -> & TraitEnvironment < 'db > { & self . 0 . table . trait_env } }
    };
}

impl_19!();