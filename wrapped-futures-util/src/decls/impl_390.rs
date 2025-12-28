macro_rules! deps {
    () => {
        Next!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl < St : ? Sized + Unpin > Unpin for Next < '_ , St > { }
    };
}

impl_390!();