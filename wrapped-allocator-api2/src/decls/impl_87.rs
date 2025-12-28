macro_rules! deps {
    () => {
        TryReserveError!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for TryReserveError { }
    };
}

impl_87!();