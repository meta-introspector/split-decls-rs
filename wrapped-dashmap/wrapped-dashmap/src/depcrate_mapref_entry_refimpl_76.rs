// Generated macro for impl_76 (impl)
macro_rules! Depcrate_mapref_entry_refimpl_76 {
() => {
// Module: crate::mapref::entry_ref
// Provides: {"impl_76"}
// Dependencies: {}
impl < 'a , 'q , K : Eq + Hash , Q , V > OccupiedEntryRef < 'a , 'q , K , Q , V > { pub (crate) fn new (shard : RwLockWriteGuardDetached < 'a > , key : & 'q Q , entry : hash_table :: OccupiedEntry < 'a , (K , V) > ,) -> Self { Self { shard , entry , key } } pub fn get (& self) -> & V { & self . entry . get () . 1 } pub fn get_mut (& mut self) -> & mut V { & mut self . entry . get_mut () . 1 } pub fn insert (& mut self , value : V) -> V { mem :: replace (self . get_mut () , value) } pub fn into_ref (self) -> RefMut < 'a , K , V > { let (k , v) = self . entry . into_mut () ; RefMut :: new (self . shard , k , v) } pub fn into_key (self) -> K where K : From < & 'q Q > , { K :: from (self . key) } pub fn key (& self) -> & 'q Q { self . key } pub fn remove (self) -> V { let ((_k , v) , _) = self . entry . remove () ; v } pub fn remove_entry (self) -> (K , V) { let ((k , v) , _) = self . entry . remove () ; (k , v) } pub fn replace_entry (self , value : V) -> (K , V) where K : From < & 'q Q > , { let (k , v) = mem :: replace (self . entry . into_mut () , (K :: from (self . key) , value)) ; (k , v) } }
};
}
