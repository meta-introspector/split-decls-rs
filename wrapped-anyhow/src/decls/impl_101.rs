macro_rules! deps {
    () => {
        BoxedKind!();
        StdError!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl BoxedKind for Box < dyn StdError + Send + Sync > { }
    };
}

impl_101!();