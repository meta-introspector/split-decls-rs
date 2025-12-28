macro_rules! deps {
    () => {
        AllFuture!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized , P > Unpin for AllFuture < '_ , S , P > { }
    };
}

impl_171!()