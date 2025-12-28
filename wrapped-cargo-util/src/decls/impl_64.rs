macro_rules! deps {
    () => {
        ProcessError!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl std :: error :: Error for ProcessError { }
    };
}

impl_64!();