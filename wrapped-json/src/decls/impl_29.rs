macro_rules! deps {
    () => {
        SeqAccess!();
        Deserializer!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a , R : 'a > SeqAccess < 'a , R > { fn new (de : & 'a mut Deserializer < R >) -> Self { SeqAccess { de , first : true } } }
    };
}

impl_29!()