macro_rules! deps {
    () => {
        WriteVectoredFuture!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < W : Unpin + ? Sized > Unpin for WriteVectoredFuture < '_ , W > { }
    };
}

impl_305!();