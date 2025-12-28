macro_rules! deps {
    () => {
        StartError!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for StartError { }
    };
}

impl_175!()