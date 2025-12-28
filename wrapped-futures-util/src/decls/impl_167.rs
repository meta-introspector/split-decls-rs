macro_rules! deps {
    () => {
        TryMaybeDone!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < Fut : TryFuture + Unpin > Unpin for TryMaybeDone < Fut > { }
    };
}

impl_167!()