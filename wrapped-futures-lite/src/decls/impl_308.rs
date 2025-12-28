macro_rules! deps {
    () => {
        WriteAllFuture!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < W : Unpin + ? Sized > Unpin for WriteAllFuture < '_ , W > { }
    };
}

impl_308!()