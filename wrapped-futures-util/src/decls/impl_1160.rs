macro_rules! deps {
    () => {
        ReadLine!();
    };
}

macro_rules! impl_1160 {
    () => {
        deps!();
        impl < R : ? Sized + Unpin > Unpin for ReadLine < '_ , R > { }
    };
}

impl_1160!();