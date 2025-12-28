macro_rules! deps {
    () => {
        ReadLineFuture!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < R : Unpin + ? Sized > Unpin for ReadLineFuture < '_ , R > { }
    };
}

impl_256!()