macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
    };
}

macro_rules! RefMutMulti {
    () => {
        deps!();
        pub struct RefMutMulti < 'a , K , V > { _guard : Arc < RwLockWriteGuardDetached < 'a > > , k : & 'a K , v : & 'a mut V , }
    };
}

RefMutMulti!();