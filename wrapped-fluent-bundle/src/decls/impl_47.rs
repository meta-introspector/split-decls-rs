macro_rules! deps {
    () => {
        ResolverError!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Error for ResolverError { }
    };
}

impl_47!();