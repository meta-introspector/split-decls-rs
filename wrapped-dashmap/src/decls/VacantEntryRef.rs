macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
        VacantEntry!();
    };
}

macro_rules! VacantEntryRef {
    () => {
        deps!();
        pub struct VacantEntryRef < 'a , 'q , K , Q , V > { shard : RwLockWriteGuardDetached < 'a > , entry : hash_table :: VacantEntry < 'a , (K , V) > , key : & 'q Q , }
    };
}

VacantEntryRef!()