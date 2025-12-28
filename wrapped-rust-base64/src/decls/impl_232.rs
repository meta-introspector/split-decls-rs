macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , test))] impl error :: Error for DecodeError { }
    };
}

impl_232!();