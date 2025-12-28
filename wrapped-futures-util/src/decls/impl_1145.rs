macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_1145 {
    () => {
        deps!();
        impl < R : ? Sized + Unpin > Unpin for Read < '_ , R > { }
    };
}

impl_1145!();