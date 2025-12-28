macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < R > std :: error :: Error for Error < R > where R : std :: fmt :: Debug { }
    };
}

impl_10!()