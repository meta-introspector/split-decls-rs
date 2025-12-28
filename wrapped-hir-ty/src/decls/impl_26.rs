macro_rules! deps {
    () => {
        Autoderef!();
        DefaultAutoderefCtx!();
        TraitEnvironment!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a , 'db > Autoderef < 'a , 'db , usize > { # [inline] pub (crate) fn new (infcx : & 'a InferCtxt < 'db > , env : & 'a TraitEnvironment < 'db > , base_ty : Ty < 'db > ,) -> Self { Self :: new_impl (DefaultAutoderefCtx { infcx , env } , base_ty) } }
    };
}

impl_26!()