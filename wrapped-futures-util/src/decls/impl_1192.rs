macro_rules! deps {
    () => {
        Seek!();
    };
}

macro_rules! impl_1192 {
    () => {
        deps!();
        impl < S : ? Sized + Unpin > Unpin for Seek < '_ , S > { }
    };
}

impl_1192!()