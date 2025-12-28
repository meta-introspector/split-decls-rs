macro_rules! deps {
    () => {
        FlushFuture!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < W : Unpin + ? Sized > Unpin for FlushFuture < '_ , W > { }
    };
}

impl_311!();