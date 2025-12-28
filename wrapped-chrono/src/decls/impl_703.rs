macro_rules! deps {
    () => {
        Error!();
        ParseWeekdayError!();
    };
}

macro_rules! impl_703 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for ParseWeekdayError { }
    };
}

impl_703!()