macro_rules! deps {
    () => {
        PositionFuture!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized , P > Unpin for PositionFuture < '_ , S , P > { }
    };
}

impl_168!();