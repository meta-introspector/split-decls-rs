macro_rules! deps {
    () => {
        SmallIndexError!();
    };
}

macro_rules! impl_769 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SmallIndexError { }
    };
}

impl_769!();