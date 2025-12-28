macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "core-error"))] impl core :: error :: Error for OutOfRange { }
    };
}

impl_22!()