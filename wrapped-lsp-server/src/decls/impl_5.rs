macro_rules! deps {
    () => {
        Request!();
        ExtractError!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl std :: error :: Error for ExtractError < Request > { }
    };
}

impl_5!()