macro_rules! deps {
    () => {
        NextFuture!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized > Unpin for NextFuture < '_ , S > { }
    };
}

impl_91!()