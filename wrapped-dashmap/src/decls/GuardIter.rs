macro_rules! deps {
    () => {
        Iter!();
        RwLockReadGuardDetached!();
    };
}

macro_rules! GuardIter {
    () => {
        deps!();
        type GuardIter < 'a , K , V > = (Arc < RwLockReadGuardDetached < 'a > > , hash_table :: Iter < 'a , (K , V) > ,) ;
    };
}

GuardIter!();