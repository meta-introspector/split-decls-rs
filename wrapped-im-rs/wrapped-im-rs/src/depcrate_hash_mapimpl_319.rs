// Generated macro for impl_319 (impl)
macro_rules! Depcrate_hash_mapimpl_319 {
() => {
// Module: crate::hash::map
// Provides: {"impl_319"}
// Dependencies: {}
impl < 'a , K , V , S > OccupiedEntry < 'a , K , V , S > where K : 'a + Hash + Eq + Clone , V : 'a + Clone , S : 'a + BuildHasher , { # [doc = " Get the key for this entry."] # [must_use] pub fn key (& self) -> & K { & self . key } # [doc = " Remove this entry from the map and return the removed mapping."] pub fn remove_entry (self) -> (K , V) { let root = PoolRef :: make_mut (& self . map . pool . 0 , & mut self . map . root) ; let result = root . remove (& self . map . pool . 0 , self . hash , 0 , & self . key) ; self . map . size -= 1 ; result . unwrap () } # [doc = " Get the current value."] # [must_use] pub fn get (& self) -> & V { & self . map . root . get (self . hash , 0 , & self . key) . unwrap () . 1 } # [doc = " Get a mutable reference to the current value."] # [must_use] pub fn get_mut (& mut self) -> & mut V { let root = PoolRef :: make_mut (& self . map . pool . 0 , & mut self . map . root) ; & mut root . get_mut (& self . map . pool . 0 , self . hash , 0 , & self . key) . unwrap () . 1 } # [doc = " Convert this entry into a mutable reference."] # [must_use] pub fn into_mut (self) -> & 'a mut V { let root = PoolRef :: make_mut (& self . map . pool . 0 , & mut self . map . root) ; & mut root . get_mut (& self . map . pool . 0 , self . hash , 0 , & self . key) . unwrap () . 1 } # [doc = " Overwrite the current value."] pub fn insert (& mut self , value : V) -> V { mem :: replace (self . get_mut () , value) } # [doc = " Remove this entry from the map and return the removed value."] pub fn remove (self) -> V { self . remove_entry () . 1 } }
};
}
