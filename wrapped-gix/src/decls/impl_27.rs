macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Sealed for TreeRefIter < '_ > { }
    };
}

impl_27!();