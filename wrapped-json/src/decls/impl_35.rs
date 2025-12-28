macro_rules! deps {
    () => {
        VariantAccess!();
        Deserializer!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a , R : 'a > VariantAccess < 'a , R > { fn new (de : & 'a mut Deserializer < R >) -> Self { VariantAccess { de } } }
    };
}

impl_35!()