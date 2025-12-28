macro_rules! deps {
    () => {
        ReadVectored!();
    };
}

macro_rules! impl_1150 {
    () => {
        deps!();
        impl < R : ? Sized + Unpin > Unpin for ReadVectored < '_ , '_ , R > { }
    };
}

impl_1150!();