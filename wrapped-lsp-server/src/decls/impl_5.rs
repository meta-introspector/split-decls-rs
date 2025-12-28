macro_rules! deps {
    () => {
        ExtractError!();
        Request!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl std :: error :: Error for ExtractError < Request > { }
    };
}

impl_5!();