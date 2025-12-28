macro_rules! deps {
    () => {
        EncodeError!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        # [cfg (not (feature = "alloc"))] impl serde :: ser :: Error for crate :: error :: EncodeError { fn custom < T > (_ : T) -> Self where T : core :: fmt :: Display , { EncodeError :: CustomError . into () } }
    };
}

impl_183!();