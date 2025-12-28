macro_rules! deps {
    () => {
        RefMutMulti!();
        RwLockWriteGuardDetached!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > RefMutMulti < 'a , K , V > { pub (crate) fn new (guard : Arc < RwLockWriteGuardDetached < 'a > > , k : & 'a K , v : & 'a mut V) -> Self { Self { _guard : guard , k , v , } } pub fn key (& self) -> & K { self . pair () . 0 } pub fn value (& self) -> & V { self . pair () . 1 } pub fn value_mut (& mut self) -> & mut V { self . pair_mut () . 1 } pub fn pair (& self) -> (& K , & V) { (self . k , self . v) } pub fn pair_mut (& mut self) -> (& K , & mut V) { (self . k , self . v) } }
    };
}

impl_57!()