macro_rules! deps {
    () => {
        OccupiedEntryRef!();
        RefMut!();
        VacantEntry!();
        OccupiedEntry!();
        RwLockWriteGuardDetached!();
        VacantEntryRef!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a , 'q , K : Eq + Hash , Q , V > VacantEntryRef < 'a , 'q , K , Q , V > { pub (crate) fn new (shard : RwLockWriteGuardDetached < 'a > , key : & 'q Q , entry : hash_table :: VacantEntry < 'a , (K , V) > ,) -> Self { Self { shard , entry , key } } pub fn insert (self , value : V) -> RefMut < 'a , K , V > where K : From < & 'q Q > , { let k = K :: from (self . key) ; let occupied = self . entry . insert ((k , value)) ; let (k , v) = occupied . into_mut () ; RefMut :: new (self . shard , k , v) } # [doc = " Sets the value of the entry with the VacantEntryRef’s key, and returns an OccupiedEntry."] pub fn insert_entry (self , value : V) -> OccupiedEntryRef < 'a , 'q , K , Q , V > where K : From < & 'q Q > , { let k = K :: from (self . key) ; let entry = self . entry . insert ((k , value)) ; OccupiedEntryRef :: new (self . shard , self . key , entry) } pub fn into_key (self) -> K where K : From < & 'q Q > , { K :: from (self . key) } pub fn key (& self) -> & 'q Q { self . key } }
    };
}

impl_48!()