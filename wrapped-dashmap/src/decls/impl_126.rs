macro_rules! deps {
    () => {
        RefMulti!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash > RefMulti < 'a , K > { pub (crate) fn new (inner : mapref :: multiple :: RefMulti < 'a , K , () >) -> Self { Self { inner } } pub fn key (& self) -> & K { self . inner . key () } }
    };
}

impl_126!()