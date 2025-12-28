macro_rules! deps {
    () => {
        IterMut!();
        RwLockWriteGuardDetached!();
    };
}

macro_rules! GuardIterMut {
    () => {
        deps!();
        type GuardIterMut < 'a , K , V > = (Arc < RwLockWriteGuardDetached < 'a > > , hash_table :: IterMut < 'a , (K , V) > ,) ;
    };
}

GuardIterMut!()