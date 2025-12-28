macro_rules! deps {
    () => {
        ParseMonthError!();
        Error!();
    };
}

macro_rules! impl_731 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "core-error"))] impl core :: error :: Error for ParseMonthError { }
    };
}

impl_731!()