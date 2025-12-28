macro_rules! deps {
    () => {
        GlobError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl std :: error :: Error for GlobError { }
    };
}

impl_6!()