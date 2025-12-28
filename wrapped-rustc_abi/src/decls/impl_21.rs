macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Eq for ExternAbi { }
    };
}

impl_21!()