macro_rules! deps {
    () => {
        OutOfRangeError!();
        Error!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , feature = "core-error"))] impl Error for OutOfRangeError { # [allow (deprecated)] fn description (& self) -> & str { "out of range error" } }
    };
}

impl_26!()