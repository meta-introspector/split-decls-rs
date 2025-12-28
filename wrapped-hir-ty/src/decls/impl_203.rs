macro_rules! deps {
    () => {
        DefaultAutoderefCtx!();
        TraitEnvironment!();
        AutoderefCtx!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < 'db > AutoderefCtx < 'db > for DefaultAutoderefCtx < '_ , 'db > { # [inline] fn infcx (& self) -> & InferCtxt < 'db > { self . infcx } # [inline] fn env (& self) -> & TraitEnvironment < 'db > { self . env } }
    };
}

impl_203!()