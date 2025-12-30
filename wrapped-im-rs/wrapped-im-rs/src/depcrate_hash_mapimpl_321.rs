// Generated macro for impl_321 (impl)
macro_rules! Depcrate_hash_mapimpl_321 {
() => {
// Module: crate::hash::map
// Provides: {"impl_321"}
// Dependencies: {}
impl < 'a , K , V , S > VacantEntry < 'a , K , V , S > where K : 'a + Hash + Eq + Clone , V : 'a + Clone , S : 'a + BuildHasher , { # [doc = " Get the key for this entry."] # [must_use] pub fn key (& self) -> & K { & self . key } # [doc = " Convert this entry into its key."] # [must_use] pub fn into_key (self) -> K { self . key } # [doc = " Insert a value into this entry."] pub fn insert (self , value : V) -> & 'a mut V { let root = PoolRef :: make_mut (& self . map . pool . 0 , & mut self . map . root) ; if root . insert (& self . map . pool . 0 , self . hash , 0 , (self . key . clone () , value)) . is_none () { self . map . size += 1 ; } & mut root . get_mut (& self . map . pool . 0 , self . hash , 0 , & self . key) . unwrap () . 1 } }
};
}
