macro_rules! deps {
    () => {
        ReadExact!();
    };
}

macro_rules! impl_1155 {
    () => {
        deps!();
        impl < R : ? Sized + Unpin > Unpin for ReadExact < '_ , R > { }
    };
}

impl_1155!();