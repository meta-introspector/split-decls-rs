macro_rules! deps {
    () => {
        Error!();
        OutOfRange!();
    };
}

macro_rules! impl_750 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "core-error"))] impl core :: error :: Error for OutOfRange { }
    };
}

impl_750!()