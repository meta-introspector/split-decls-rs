macro_rules! deps {
    () => {
        ParseWeekdayError!();
        Error!();
    };
}

macro_rules! impl_703 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for ParseWeekdayError { }
    };
}

impl_703!();