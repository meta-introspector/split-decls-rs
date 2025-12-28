macro_rules! deps {
    () => {
        LocalFutureObj!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > Unpin for LocalFutureObj < '_ , T > { }
    };
}

impl_31!()