macro_rules! deps {
    () => {
        Error!();
        ParseWeekdayError!();
    };
}

macro_rules! impl_702 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "core-error"))] impl core :: error :: Error for ParseWeekdayError { }
    };
}

impl_702!();