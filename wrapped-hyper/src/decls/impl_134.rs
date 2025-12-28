macro_rules! deps {
    () => {
        Error!();
        InvalidReasonPhrase!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl std :: error :: Error for InvalidReasonPhrase { }
    };
}

impl_134!();