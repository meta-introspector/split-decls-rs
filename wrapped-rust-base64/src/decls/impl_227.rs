macro_rules! deps {
    () => {
        EncodeSliceError!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , test))] impl error :: Error for EncodeSliceError { }
    };
}

impl_227!()