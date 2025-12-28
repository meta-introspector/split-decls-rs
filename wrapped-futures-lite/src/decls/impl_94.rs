macro_rules! deps {
    () => {
        TryNextFuture!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized > Unpin for TryNextFuture < '_ , S > { }
    };
}

impl_94!()