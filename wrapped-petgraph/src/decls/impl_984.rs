macro_rules! deps {
    () => {
        MatrixError!();
    };
}

macro_rules! impl_984 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] impl core :: error :: Error for MatrixError { }
    };
}

impl_984!()