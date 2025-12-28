macro_rules! deps {
    () => {
        Autoderef!();
        TraitEnvironment!();
        DefaultAutoderefCtx!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < 'a , 'db > Autoderef < 'a , 'db > { # [inline] pub (crate) fn new_with_tracking (infcx : & 'a InferCtxt < 'db > , env : & 'a TraitEnvironment < 'db > , base_ty : Ty < 'db > ,) -> Self { Self :: new_impl (DefaultAutoderefCtx { infcx , env } , base_ty) } }
    };
}

impl_210!()