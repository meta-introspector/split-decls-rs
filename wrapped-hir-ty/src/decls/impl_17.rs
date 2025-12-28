macro_rules! deps {
    () => {
        AutoderefCtx!();
        DefaultAutoderefCtx!();
        TraitEnvironment!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'db > AutoderefCtx < 'db > for DefaultAutoderefCtx < '_ , 'db > { # [inline] fn infcx (& self) -> & InferCtxt < 'db > { self . infcx } # [inline] fn env (& self) -> & TraitEnvironment < 'db > { self . env } }
    };
}

impl_17!();