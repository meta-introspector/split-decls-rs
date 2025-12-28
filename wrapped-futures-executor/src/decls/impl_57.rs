macro_rules! deps {
    () => {
        EnterError!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl std :: error :: Error for EnterError { }
    };
}

impl_57!()