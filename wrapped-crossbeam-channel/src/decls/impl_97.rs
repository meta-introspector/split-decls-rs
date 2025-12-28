macro_rules! deps {
    () => {
        SelectTimeoutError!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl error :: Error for SelectTimeoutError { }
    };
}

impl_97!()