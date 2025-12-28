macro_rules! deps {
    () => {
        UnitVariantAccess!();
        Deserializer!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a , R : 'a > UnitVariantAccess < 'a , R > { fn new (de : & 'a mut Deserializer < R >) -> Self { UnitVariantAccess { de } } }
    };
}

impl_39!();