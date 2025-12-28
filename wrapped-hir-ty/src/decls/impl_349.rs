macro_rules! deps {
    () => {
        AutoderefCtx!();
        DefaultAutoderefCtx!();
        TraitEnvironment!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < 'db > AutoderefCtx < 'db > for DefaultAutoderefCtx < '_ , 'db > { # [inline] fn infcx (& self) -> & InferCtxt < 'db > { self . infcx } # [inline] fn env (& self) -> & TraitEnvironment < 'db > { self . env } }
    };
}

impl_349!();