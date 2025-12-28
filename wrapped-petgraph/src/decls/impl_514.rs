macro_rules! deps {
    () => {
        CsrError!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] impl core :: error :: Error for CsrError { }
    };
}

impl_514!()