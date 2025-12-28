macro_rules! deps {
    () => {
        InferenceContextAutoderefCtx!();
        InferenceContextAutoderef!();
        InferenceContext!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , 'b , 'db > InferenceContextAutoderef < 'a , 'b , 'db > { # [inline] pub (crate) fn new_from_inference_context (ctx : & 'a mut InferenceContext < 'b , 'db > , base_ty : Ty < 'db > ,) -> Self { Self :: new_impl (InferenceContextAutoderefCtx (ctx) , base_ty) } # [inline] pub (crate) fn ctx (& mut self) -> & mut InferenceContext < 'b , 'db > { self . ctx . 0 } }
    };
}

impl_25!();