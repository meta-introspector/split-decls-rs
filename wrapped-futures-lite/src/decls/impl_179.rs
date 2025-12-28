macro_rules! deps {
    () => {
        TryForEachFuture!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized , F > Unpin for TryForEachFuture < '_ , S , F > { }
    };
}

impl_179!();