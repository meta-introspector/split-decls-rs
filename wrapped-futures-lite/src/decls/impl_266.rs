macro_rules! deps {
    () => {
        ReadFuture!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < R : Unpin + ? Sized > Unpin for ReadFuture < '_ , R > { }
    };
}

impl_266!()