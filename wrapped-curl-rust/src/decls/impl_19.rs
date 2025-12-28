macro_rules! deps {
    () => {
        Error!();
        FormError!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl error :: Error for FormError { }
    };
}

impl_19!();