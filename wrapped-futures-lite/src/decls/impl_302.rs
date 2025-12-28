macro_rules! deps {
    () => {
        WriteFuture!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < W : Unpin + ? Sized > Unpin for WriteFuture < '_ , W > { }
    };
}

impl_302!();