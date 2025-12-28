macro_rules! deps {
    () => {
        IterNames!();
        Iter!();
        Flags!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < B : Flags > Iter < B > { pub (crate) fn new (flags : & B) -> Self { Iter { inner : IterNames :: new (flags) , done : false , } } }
    };
}

impl_1!()