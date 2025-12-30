// Generated macro for impl_74 (impl)
macro_rules! Depcrate_mapref_entry_refimpl_74 {
() => {
// Module: crate::mapref::entry_ref
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a , 'q , K : Eq + Hash , Q , V > VacantEntryRef < 'a , 'q , K , Q , V > { pub (crate) fn new (shard : RwLockWriteGuardDetached < 'a > , key : & 'q Q , entry : hash_table :: VacantEntry < 'a , (K , V) > ,) -> Self { Self { shard , entry , key } } pub fn insert (self , value : V) -> RefMut < 'a , K , V > where K : From < & 'q Q > , { let k = K :: from (self . key) ; let occupied = self . entry . insert ((k , value)) ; let (k , v) = occupied . into_mut () ; RefMut :: new (self . shard , k , v) } # [doc = " Sets the value of the entry with the VacantEntryRef’s key, and returns an OccupiedEntry."] pub fn insert_entry (self , value : V) -> OccupiedEntryRef < 'a , 'q , K , Q , V > where K : From < & 'q Q > , { let k = K :: from (self . key) ; let entry = self . entry . insert ((k , value)) ; OccupiedEntryRef :: new (self . shard , self . key , entry) } pub fn into_key (self) -> K where K : From < & 'q Q > , { K :: from (self . key) } pub fn key (& self) -> & 'q Q { self . key } }
};
}
