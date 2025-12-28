macro_rules! deps {
    () => {
        RefMulti!();
        RwLockReadGuardDetached!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > RefMulti < 'a , K , V > { pub (crate) fn new (guard : Arc < RwLockReadGuardDetached < 'a > > , k : & 'a K , v : & 'a V) -> Self { Self { _guard : guard , k , v , } } pub fn key (& self) -> & K { self . pair () . 0 } pub fn value (& self) -> & V { self . pair () . 1 } pub fn pair (& self) -> (& K , & V) { (self . k , self . v) } }
    };
}

impl_54!();