macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        # [cfg (not (feature = "alloc"))] impl serde :: de :: Error for crate :: error :: DecodeError { fn custom < T > (_ : T) -> Self where T : core :: fmt :: Display , { DecodeError :: CustomError . into () } }
    };
}

impl_178!();