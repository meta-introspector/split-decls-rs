macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Error { }
    };
}

impl_133!()