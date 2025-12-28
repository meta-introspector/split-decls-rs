macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_883 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Error { }
    };
}

impl_883!()