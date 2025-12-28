macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Eq for Block { }
    };
}

impl_56!()