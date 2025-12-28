macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < Fut : Unpin > Unpin for SelectAll < Fut > { }
    };
}

impl_232!();