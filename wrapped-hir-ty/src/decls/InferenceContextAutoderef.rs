macro_rules! deps {
    () => {
        AutoderefKind!();
        GeneralAutoderef!();
        InferenceContextAutoderefCtx!();
    };
}

macro_rules! InferenceContextAutoderef {
    () => {
        deps!();
        pub (crate) type InferenceContextAutoderef < 'a , 'b , 'db , Steps = Vec < (Ty < 'db > , AutoderefKind) > > = GeneralAutoderef < 'db , InferenceContextAutoderefCtx < 'a , 'b , 'db > , Steps > ;
    };
}

InferenceContextAutoderef!()