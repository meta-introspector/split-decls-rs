macro_rules! deps {
    () => {
        ReadToEnd!();
    };
}

macro_rules! impl_1167 {
    () => {
        deps!();
        impl < R : ? Sized + Unpin > Unpin for ReadToEnd < '_ , R > { }
    };
}

impl_1167!();