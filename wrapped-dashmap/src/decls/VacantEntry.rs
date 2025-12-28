macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        pub struct VacantEntry < 'a , K , V > { shard : RwLockWriteGuardDetached < 'a > , key : K , entry : hash_table :: VacantEntry < 'a , (K , V) > , }
    };
}

VacantEntry!()