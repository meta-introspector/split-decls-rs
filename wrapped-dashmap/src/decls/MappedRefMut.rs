macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
    };
}

macro_rules! MappedRefMut {
    () => {
        deps!();
        pub struct MappedRefMut < 'a , K , T : ? Sized > { _guard : RwLockWriteGuardDetached < 'a > , k : & 'a K , v : & 'a mut T , }
    };
}

MappedRefMut!();