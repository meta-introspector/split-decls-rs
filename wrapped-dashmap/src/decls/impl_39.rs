macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
        RefMut!();
        OccupiedEntry!();
        VacantEntry!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > VacantEntry < 'a , K , V > { pub (crate) fn new (shard : RwLockWriteGuardDetached < 'a > , key : K , entry : hash_table :: VacantEntry < 'a , (K , V) > ,) -> Self { Self { shard , key , entry } } pub fn insert (self , value : V) -> RefMut < 'a , K , V > { let occupied = self . entry . insert ((self . key , value)) ; let (k , v) = occupied . into_mut () ; RefMut :: new (self . shard , k , v) } # [doc = " Sets the value of the entry with the VacantEntry’s key, and returns an OccupiedEntry."] pub fn insert_entry (self , value : V) -> OccupiedEntry < 'a , K , V > where K : Clone , { let entry = self . entry . insert ((self . key . clone () , value)) ; OccupiedEntry :: new (self . shard , self . key , entry) } pub fn into_key (self) -> K { self . key } pub fn key (& self) -> & K { & self . key } }
    };
}

impl_39!();