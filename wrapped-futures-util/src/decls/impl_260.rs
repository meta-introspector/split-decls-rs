macro_rules! deps {
    () => {
        SelectOk!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < Fut : Unpin > Unpin for SelectOk < Fut > { }
    };
}

impl_260!();