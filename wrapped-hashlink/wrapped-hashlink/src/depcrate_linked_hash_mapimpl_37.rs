// Generated macro for impl_37 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_37 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , K , V , S > VacantEntry < 'a , K , V , S > { # [inline] pub fn key (& self) -> & K { & self . key } # [inline] pub fn into_key (self) -> K { self . key } # [doc = " Insert's the key for this vacant entry paired with the given value as a new entry at the"] # [doc = " *back* of the internal linked list."] # [inline] pub fn insert (self , value : V) -> & 'a mut V where K : Hash , S : BuildHasher , { self . raw_entry . insert (self . key , value) . 1 } }
};
}
