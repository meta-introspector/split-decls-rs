macro_rules! deps {
    () => {
        OutOfRange!();
        Error!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for OutOfRange { }
    };
}

impl_749!();