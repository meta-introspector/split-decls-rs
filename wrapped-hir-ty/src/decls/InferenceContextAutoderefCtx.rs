macro_rules! deps {
    () => {
        InferenceContext!();
    };
}

macro_rules! InferenceContextAutoderefCtx {
    () => {
        deps!();
        pub (crate) struct InferenceContextAutoderefCtx < 'a , 'b , 'db > (& 'a mut InferenceContext < 'b , 'db >) ;
    };
}

InferenceContextAutoderefCtx!()