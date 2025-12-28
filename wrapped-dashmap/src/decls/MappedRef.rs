macro_rules! deps {
    () => {
        RwLockReadGuardDetached!();
    };
}

macro_rules! MappedRef {
    () => {
        deps!();
        pub struct MappedRef < 'a , K , T : ? Sized > { _guard : RwLockReadGuardDetached < 'a > , k : & 'a K , v : & 'a T , }
    };
}

MappedRef!();