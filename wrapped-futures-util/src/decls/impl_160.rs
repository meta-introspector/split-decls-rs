macro_rules! deps {
    () => {
        MaybeDone!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < Fut : Future + Unpin > Unpin for MaybeDone < Fut > { }
    };
}

impl_160!();