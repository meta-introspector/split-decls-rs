macro_rules! deps {
    () => {
        Empty!();
        Error!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Empty { }
    };
}

impl_122!()