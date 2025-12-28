macro_rules! deps {
    () => {
        ParseWeekdayError!();
        Error!();
    };
}

macro_rules! impl_702 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "core-error"))] impl core :: error :: Error for ParseWeekdayError { }
    };
}

impl_702!()