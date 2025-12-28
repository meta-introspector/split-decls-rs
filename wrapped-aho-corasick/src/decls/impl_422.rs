macro_rules! deps {
    () => {
        SmallIndexError!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SmallIndexError { }
    };
}

impl_422!();