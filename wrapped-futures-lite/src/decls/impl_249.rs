macro_rules! deps {
    () => {
        FillBuf!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < R : ? Sized > Unpin for FillBuf < '_ , R > { }
    };
}

impl_249!();