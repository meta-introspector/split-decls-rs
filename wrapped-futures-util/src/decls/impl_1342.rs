macro_rules! deps {
    () => {
        Aborted!();
    };
}

macro_rules! impl_1342 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Aborted { }
    };
}

impl_1342!();