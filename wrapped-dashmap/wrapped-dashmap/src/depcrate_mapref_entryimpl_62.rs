// Generated macro for impl_62 (impl)
macro_rules! Depcrate_mapref_entryimpl_62 {
() => {
// Module: crate::mapref::entry
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > OccupiedEntry < 'a , K , V > { pub (crate) fn new (shard : RwLockWriteGuardDetached < 'a > , key : K , entry : hash_table :: OccupiedEntry < 'a , (K , V) > ,) -> Self { Self { shard , entry , key } } pub fn get (& self) -> & V { & self . entry . get () . 1 } pub fn get_mut (& mut self) -> & mut V { & mut self . entry . get_mut () . 1 } pub fn insert (& mut self , value : V) -> V { mem :: replace (self . get_mut () , value) } pub fn into_ref (self) -> RefMut < 'a , K , V > { let (k , v) = self . entry . into_mut () ; RefMut :: new (self . shard , k , v) } pub fn into_key (self) -> K { self . key } pub fn key (& self) -> & K { & self . entry . get () . 0 } pub fn remove (self) -> V { let ((_k , v) , _) = self . entry . remove () ; v } pub fn remove_entry (self) -> (K , V) { let ((k , v) , _) = self . entry . remove () ; (k , v) } pub fn replace_entry (self , value : V) -> (K , V) { let (k , v) = mem :: replace (self . entry . into_mut () , (self . key , value)) ; (k , v) } }
};
}
