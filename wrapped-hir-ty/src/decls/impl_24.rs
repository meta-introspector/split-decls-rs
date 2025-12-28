macro_rules! deps {
    () => {
        TraitEnvironment!();
        Autoderef!();
        DefaultAutoderefCtx!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a , 'db > Autoderef < 'a , 'db > { # [inline] pub (crate) fn new_with_tracking (infcx : & 'a InferCtxt < 'db > , env : & 'a TraitEnvironment < 'db > , base_ty : Ty < 'db > ,) -> Self { Self :: new_impl (DefaultAutoderefCtx { infcx , env } , base_ty) } }
    };
}

impl_24!()