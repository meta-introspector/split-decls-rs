macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + 'a > Iter < 'a , K > { pub (crate) fn new (inner : crate :: iter :: Iter < 'a , K , () >) -> Self { Self { inner } } }
    };
}

impl_21!()