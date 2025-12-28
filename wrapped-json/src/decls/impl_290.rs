macro_rules! deps {
    () => {
        BorrowedCowStrDeserializer!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < 'de > BorrowedCowStrDeserializer < 'de > { fn new (value : Cow < 'de , str >) -> Self { BorrowedCowStrDeserializer { value } } }
    };
}

impl_290!()