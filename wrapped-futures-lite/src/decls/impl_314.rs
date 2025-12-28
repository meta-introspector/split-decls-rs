macro_rules! deps {
    () => {
        CloseFuture!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < W : Unpin + ? Sized > Unpin for CloseFuture < '_ , W > { }
    };
}

impl_314!();