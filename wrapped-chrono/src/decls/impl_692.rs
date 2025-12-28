macro_rules! deps {
    () => {
        DurationRound!();
        RoundingError!();
        Error!();
    };
}

macro_rules! impl_692 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for RoundingError { # [allow (deprecated)] fn description (& self) -> & str { "error from rounding or truncating with DurationRound" } }
    };
}

impl_692!();