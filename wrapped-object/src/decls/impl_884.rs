macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_884 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , core_error))] impl core :: error :: Error for Error { }
    };
}

impl_884!()