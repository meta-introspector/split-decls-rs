macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_677 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl error :: Error for Error { }
    };
}

impl_677!();