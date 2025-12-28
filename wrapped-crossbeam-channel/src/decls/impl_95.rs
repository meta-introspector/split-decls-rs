macro_rules! deps {
    () => {
        TrySelectError!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl error :: Error for TrySelectError { }
    };
}

impl_95!()