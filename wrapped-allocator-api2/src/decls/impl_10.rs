macro_rules! deps {
    () => {
        AllocError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for AllocError { }
    };
}

impl_10!()