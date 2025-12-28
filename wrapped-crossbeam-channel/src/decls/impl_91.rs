macro_rules! deps {
    () => {
        RecvTimeoutError!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl error :: Error for RecvTimeoutError { }
    };
}

impl_91!()