macro_rules! deps {
    () => {
        AnyFuture!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized , P > Unpin for AnyFuture < '_ , S , P > { }
    };
}

impl_174!();