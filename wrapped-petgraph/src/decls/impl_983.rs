macro_rules! deps {
    () => {
        MatrixError!();
    };
}

macro_rules! impl_983 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for MatrixError { }
    };
}

impl_983!();