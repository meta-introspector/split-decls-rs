macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_728 {
    () => {
        deps!();
        impl Eq for Signature < '_ > { }
    };
}

impl_728!()