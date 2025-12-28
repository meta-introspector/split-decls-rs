macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Error { }
    };
}

impl_215!()