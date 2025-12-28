macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl StableOrd for ExternAbi { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
    };
}

impl_24!()