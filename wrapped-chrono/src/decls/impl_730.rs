macro_rules! deps {
    () => {
        ParseMonthError!();
        Error!();
    };
}

macro_rules! impl_730 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for ParseMonthError { }
    };
}

impl_730!();