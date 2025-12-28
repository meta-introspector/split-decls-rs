macro_rules! deps {
    () => {
        RoundingError!();
        Error!();
        DurationRound!();
    };
}

macro_rules! impl_693 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "core-error"))] impl core :: error :: Error for RoundingError { # [allow (deprecated)] fn description (& self) -> & str { "error from rounding or truncating with DurationRound" } }
    };
}

impl_693!()