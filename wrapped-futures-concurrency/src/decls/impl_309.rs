macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < E : Error , const N : usize > std :: error :: Error for AggregateError < E , N > { }
    };
}

impl_309!();