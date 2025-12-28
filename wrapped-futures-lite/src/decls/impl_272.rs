macro_rules! deps {
    () => {
        ReadToEndFuture!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < R : Unpin + ? Sized > Unpin for ReadToEndFuture < '_ , R > { }
    };
}

impl_272!()