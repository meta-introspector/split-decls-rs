macro_rules! deps {
    () => {
        TryReserveError!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl core :: error :: Error for TryReserveError { }
    };
}

impl_196!();