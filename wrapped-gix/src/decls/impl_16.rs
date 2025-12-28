macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Sealed for ObjectId { }
    };
}

impl_16!()