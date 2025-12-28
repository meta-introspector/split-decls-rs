macro_rules! deps {
    () => {
        Error!();
        MultiError!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl error :: Error for MultiError { }
    };
}

impl_14!()