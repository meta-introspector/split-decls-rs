macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash > Ref < 'a , K > { pub (crate) fn new (inner : mapref :: one :: Ref < 'a , K , () >) -> Self { Self { inner } } pub fn key (& self) -> & K { self . inner . key () } }
    };
}

impl_130!()