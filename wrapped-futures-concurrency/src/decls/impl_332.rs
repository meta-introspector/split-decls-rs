macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < E : Error > Error for AggregateError < E > { }
    };
}

impl_332!();