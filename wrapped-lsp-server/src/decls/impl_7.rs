macro_rules! deps {
    () => {
        ExtractError!();
        Notification!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl std :: error :: Error for ExtractError < Notification > { }
    };
}

impl_7!()