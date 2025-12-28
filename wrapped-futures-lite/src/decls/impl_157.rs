macro_rules! deps {
    () => {
        NthFuture!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < S : Unpin + ? Sized > Unpin for NthFuture < '_ , S > { }
    };
}

impl_157!()