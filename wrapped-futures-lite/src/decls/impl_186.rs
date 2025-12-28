macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized > Unpin for Drain < '_ , S > { }
    };
}

impl_186!();