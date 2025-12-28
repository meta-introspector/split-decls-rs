macro_rules! deps {
    () => {
        TryReserveError!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl core :: error :: Error for TryReserveError { }
    };
}

impl_20!()