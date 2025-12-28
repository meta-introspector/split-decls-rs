macro_rules! deps {
    () => {
        InvalidLengthError!();
        Error!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidLengthError { }
    };
}

impl_46!()