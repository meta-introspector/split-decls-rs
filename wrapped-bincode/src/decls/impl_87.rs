macro_rules! deps {
    () => {
        IoReader!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < R > IoReader < R > { pub const fn new (reader : R) -> Self { Self { reader } } }
    };
}

impl_87!()