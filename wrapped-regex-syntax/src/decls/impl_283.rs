macro_rules! deps {
    () => {
        Error!();
        UnicodeWordError!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for UnicodeWordError { }
    };
}

impl_283!();