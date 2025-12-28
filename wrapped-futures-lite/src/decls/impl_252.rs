macro_rules! deps {
    () => {
        ReadUntilFuture!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < R : Unpin + ? Sized > Unpin for ReadUntilFuture < '_ , R > { }
    };
}

impl_252!()