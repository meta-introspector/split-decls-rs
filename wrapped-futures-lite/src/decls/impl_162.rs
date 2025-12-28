macro_rules! deps {
    () => {
        FindFuture!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized , P > Unpin for FindFuture < '_ , S , P > { }
    };
}

impl_162!();