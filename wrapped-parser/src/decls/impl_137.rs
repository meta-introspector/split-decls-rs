macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl std :: error :: Error for Error { }
    };
}

impl_137!();