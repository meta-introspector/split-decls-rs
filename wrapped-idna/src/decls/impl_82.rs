macro_rules! deps {
    () => {
        Errors!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Errors { }
    };
}

impl_82!();