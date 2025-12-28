macro_rules! deps {
    () => {
        ReadToStringFuture!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < R : Unpin + ? Sized > Unpin for ReadToStringFuture < '_ , R > { }
    };
}

impl_275!()