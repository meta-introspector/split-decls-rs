macro_rules! deps {
    () => {
        TryGetError!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for TryGetError { }
    };
}

impl_283!();