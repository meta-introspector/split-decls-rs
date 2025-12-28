macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_1067 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , core_error))] impl core :: error :: Error for Error { }
    };
}

impl_1067!()