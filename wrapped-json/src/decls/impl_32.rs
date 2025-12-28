macro_rules! deps {
    () => {
        MapAccess!();
        Deserializer!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a , R : 'a > MapAccess < 'a , R > { fn new (de : & 'a mut Deserializer < R >) -> Self { MapAccess { de , first : true } } }
    };
}

impl_32!();