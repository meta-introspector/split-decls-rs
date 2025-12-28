macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_1066 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl error :: Error for Error { }
    };
}

impl_1066!();