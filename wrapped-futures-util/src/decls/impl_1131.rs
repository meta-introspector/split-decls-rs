macro_rules! deps {
    () => {
        Flush!();
    };
}

macro_rules! impl_1131 {
    () => {
        deps!();
        impl < W : ? Sized + Unpin > Unpin for Flush < '_ , W > { }
    };
}

impl_1131!();