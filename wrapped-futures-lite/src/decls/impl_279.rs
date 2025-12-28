macro_rules! deps {
    () => {
        ReadExactFuture!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < R : Unpin + ? Sized > Unpin for ReadExactFuture < '_ , R > { }
    };
}

impl_279!();