macro_rules! deps {
    () => {
        InvalidReasonPhrase!();
        Error!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl std :: error :: Error for InvalidReasonPhrase { }
    };
}

impl_134!()