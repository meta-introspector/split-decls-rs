macro_rules! deps {
    () => {
        WriteAll!();
    };
}

macro_rules! impl_1237 {
    () => {
        deps!();
        impl < W : ? Sized + Unpin > Unpin for WriteAll < '_ , W > { }
    };
}

impl_1237!();