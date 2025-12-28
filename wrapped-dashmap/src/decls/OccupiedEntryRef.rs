macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
        OccupiedEntry!();
    };
}

macro_rules! OccupiedEntryRef {
    () => {
        deps!();
        pub struct OccupiedEntryRef < 'a , 'q , K , Q , V > { shard : RwLockWriteGuardDetached < 'a > , entry : hash_table :: OccupiedEntry < 'a , (K , V) > , key : & 'q Q , }
    };
}

OccupiedEntryRef!();