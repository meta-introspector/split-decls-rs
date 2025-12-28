macro_rules! deps {
    () => {
        RetryError!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for RetryError { }
    };
}

impl_307!()