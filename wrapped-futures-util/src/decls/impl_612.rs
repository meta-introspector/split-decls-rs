macro_rules! deps {
    () => {
        TryNext!();
    };
}

macro_rules! impl_612 {
    () => {
        deps!();
        impl < St : ? Sized + Unpin > Unpin for TryNext < '_ , St > { }
    };
}

impl_612!();