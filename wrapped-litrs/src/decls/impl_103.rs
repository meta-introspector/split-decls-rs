macro_rules! deps {
    () => {
        InvalidToken!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl std :: error :: Error for InvalidToken { }
    };
}

impl_103!()