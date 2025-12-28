macro_rules! deps {
    () => {
        Errors!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] impl core :: error :: Error for Errors { }
    };
}

impl_83!();