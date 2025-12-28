macro_rules! deps {
    () => {
        SeekFuture!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized > Unpin for SeekFuture < '_ , S > { }
    };
}

impl_297!()