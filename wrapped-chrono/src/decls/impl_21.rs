macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for OutOfRange { }
    };
}

impl_21!()