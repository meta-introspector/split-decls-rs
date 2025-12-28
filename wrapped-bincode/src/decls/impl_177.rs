macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl serde :: de :: Error for crate :: error :: DecodeError { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { use alloc :: string :: ToString ; Self :: OtherString (msg . to_string ()) } }
    };
}

impl_177!();