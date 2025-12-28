macro_rules! deps {
    () => {
        ReadUntil!();
    };
}

macro_rules! impl_1181 {
    () => {
        deps!();
        impl < R : ? Sized + Unpin > Unpin for ReadUntil < '_ , R > { }
    };
}

impl_1181!();