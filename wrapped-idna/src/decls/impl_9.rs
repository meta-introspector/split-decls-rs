macro_rules! deps {
    () => {
        Errors!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Errors { }
    };
}

impl_9!()