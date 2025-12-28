macro_rules! deps {
    () => {
        EncodeError!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl serde :: ser :: Error for crate :: error :: EncodeError { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { use alloc :: string :: ToString ; Self :: OtherString (msg . to_string ()) } }
    };
}

impl_182!();