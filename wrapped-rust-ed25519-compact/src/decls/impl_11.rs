macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Error { }
    };
}

impl_11!();