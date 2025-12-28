macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        pub struct OccupiedEntry < 'a , K , V > { shard : RwLockWriteGuardDetached < 'a > , entry : hash_table :: OccupiedEntry < 'a , (K , V) > , key : K , }
    };
}

OccupiedEntry!()