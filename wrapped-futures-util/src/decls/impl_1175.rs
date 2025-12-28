macro_rules! deps {
    () => {
        ReadToString!();
    };
}

macro_rules! impl_1175 {
    () => {
        deps!();
        impl < R : ? Sized + Unpin > Unpin for ReadToString < '_ , R > { }
    };
}

impl_1175!()