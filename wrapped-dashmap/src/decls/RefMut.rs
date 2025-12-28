macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
    };
}

macro_rules! RefMut {
    () => {
        deps!();
        pub struct RefMut < 'a , K , V > { guard : RwLockWriteGuardDetached < 'a > , k : & 'a K , v : & 'a mut V , }
    };
}

RefMut!();