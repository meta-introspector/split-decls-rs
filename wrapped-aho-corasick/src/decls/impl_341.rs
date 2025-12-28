macro_rules! deps {
    () => {
        BuildError!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BuildError { }
    };
}

impl_341!()