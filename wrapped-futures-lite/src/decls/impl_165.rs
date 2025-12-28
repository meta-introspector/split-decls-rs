macro_rules! deps {
    () => {
        FindMapFuture!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized , F > Unpin for FindMapFuture < '_ , S , F > { }
    };
}

impl_165!();